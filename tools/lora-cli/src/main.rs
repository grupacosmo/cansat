use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
};

use clap::Parser;
use eyre::{eyre, Result, WrapErr};
use once_cell::sync::Lazy;
use regex::{Regex};
use serialport::{SerialPort, SerialPortType, UsbPortInfo};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Subcommand)]
enum Cmd {
    /// Lists available ports
    Ports,
    /// Check connection with device
    Conntest(PortArgs),
    /// Send custom commands from command.txt
    Command(PortArgs),
    /// Start a receive loop
    Receive(PortArgs),
}

#[derive(Debug, clap::Parser)]
struct PortArgs {
    /// Serial port to open
    #[arg(short, long)]
    port: String,
    /// Port baudrate.   
    /// Available values: 9600, 14400, 19200, 38400, 57600, 76800, 115200, 230400
    #[arg(short, long, default_value = "9600")]
    baudrate: u32,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();
    match args.cmd {
        Cmd::Ports => list_ports(),
        Cmd::Conntest(args) => connection_test(args),
        Cmd::Command(args) => send_command(args),
        Cmd::Receive(args) => receive(args),
    }?;
    Ok(())
}

fn list_ports() -> Result<()> {
    let ports =
        serialport::available_ports().wrap_err("Couldn't get the list of available ports")?;
    for port in ports {
        let port_type = match port.port_type {
            SerialPortType::UsbPort(UsbPortInfo { product, .. }) => product,
            _ => None,
        };
        println!(
            "{} - {}",
            port.port_name,
            port_type.unwrap_or_else(|| "Unknown".to_string())
        )
    }
    Ok(())
}

fn open_port(args: &PortArgs) -> Result<Box<dyn SerialPort>> {
    serialport::new(&args.port, args.baudrate)
        .timeout(Duration::from_secs(1))
        .open()
        .wrap_err("Failed to open serial port")
}

fn connection_test(args: PortArgs) -> Result<()> {
    eprintln!("Connection test");

    let port = open_port(&args)?;
    let mut lora = Lora::new(port);

    lora.transmit(b"AT\r\n")
        .wrap_err("Connection test failed")?;

    Ok(())
}

fn send_command(args: PortArgs) -> Result<()> {
    eprintln!("Sending custom command...");

    let file = File::open("tools/lora-cli/commands.txt")?;
    let reader = BufReader::new(file);

    let port = open_port(&args)?;
    let mut lora = Lora::new(port);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                eprintln!("{line}");
                lora.transmit(format!("{line}\r\n").as_bytes())
                    .wrap_err(format!("Failed to send {line} command"))?;
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
    Ok(())
}

fn receive(args: PortArgs) -> Result<()> {
    eprintln!("Configuring as receiver...");

    let port = open_port(&args)?;
    let mut lora = Lora::new(port);

    lora.transmit(b"AT+MODE=TEST\r\n")
        .wrap_err("Failed to set test mode")?;

    lora.transmit(b"AT+TEST=RXLRPKT\r\n")
        .wrap_err("Failed to set continuous RX mode")?;

    eprintln!("Listening...");

    for result in lora.listen()? {
        // TODO: add parsing function
        match result {
            Ok(msg) => println!("{msg}"),
            Err(e) => eprintln!("{e}"),
        }
    }
    Ok(())
}

struct Lora {
    port: BufReader<Box<dyn SerialPort>>,
}

impl Lora {
    fn new(port: Box<dyn SerialPort>) -> Self {
        // We use capacity 1 to drastically improve the performance on Windows
        Self {
            port: BufReader::with_capacity(1, port),
        }
    }

    fn receive(&mut self) -> Result<String> {
        let mut response = String::new();
        self.port
            .read_line(&mut response)
            .wrap_err("Failed to read message")?;
        eprintln!("{}", &response);
        validate_success_response(&response)?;
        Ok(response)
    }

    fn send(&mut self, input: &[u8]) -> Result<usize> {
        self.port
            .get_mut()
            .write(input)
            .wrap_err("Failed to write message")
    }

    fn transmit(&mut self, input: &[u8]) -> Result<String> {
        self.send(input)?;
        self.receive()
    }

    fn listen(mut self) -> Result<impl Iterator<Item = Result<String>>> {
        self.port
            .get_mut()
            .set_timeout(Duration::from_secs(60 * 60))
            .wrap_err("Failed to disable timeout")?;

        let iter = self.port.lines().map(|result| {
            let response = result.wrap_err("Failed to read message")?;
            validate_success_response(&response)?;
            Ok(response)
        });
        Ok(iter)
    }
}

enum Parsed {
    SignalQuality { rssi: i32, snr: i32 },
    Data(String),
}

fn parse_received_message(input: &str) -> Result<String> {
    // let signal_strength_re = r"RSSI:(-?\d+)";
    // let signal_to_noise_re = r"SNR:(-?\d+)";
    // let hex_message_re = r"RX\s*(\w+)";

    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"RSSI:(-?\d+),\s*SNR:(-?\d+)|RX\s*(\w+)").unwrap());

    let captures = RE
        .captures(input)
        .ok_or_else(|| eyre!("Failed to capture anything"))?;

    let signal_strength_dBm: Option<i32> = captures
        .get(1) // Option<Match>
        .map(|rssi| rssi.as_str().parse()) // Option<Result<i32, _>
        .transpose() // Result<Option<i32>, _>
        .wrap_err("Failed to parse rssi")?;

    let signal_to_noise_dB: Option<i32> = captures
        .get(2) // Option<Match>
        .map(|snr| snr.as_str().parse()) // Option<Result<i32, _>
        .transpose() // Result<Option<i32>, _>
        .wrap_err("Failed to parse snr")?;

    let bytes = captures
        .get(3)
        .map(|rx| hex::decode(rx.as_str()))
        .transpose()
        .wrap_err("Failed to decode rx hex")?;

    let message = bytes
        .map(|bytes| String::from_utf8(bytes))
        .transpose()
        .wrap_err("Failed to parse rx")?;

    let mut msg = String::new();

    if signal_strength_dBm != None && signal_to_noise_dB != None {
        msg = format!(
            "Signal strength: {signal_strength_dBm:?} dBm, Noise level: {signal_to_noise_dB:?} dB"
        )
    } else if message != None {
        // TODO: if ID matches config, use println! to save this to file
        // Do it here or in format_cansat_data()
        println!("{message:?}");

        // msg = format!("Message: {string_message}");

        // TODO: if failed to format cansat data, return string message instead
        msg = format_cansat_data(message.unwrap())
    }
    // This is for displaying nice formatted message with live data
    eprintln!("{msg}");
    Ok(msg)
}

fn format_cansat_data(data: String) -> String {
    data
}

fn parse_lora_error(input: &str) -> Option<i32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"ERROR\((-?\d+)\)").unwrap());
    RE.captures(input)
        .and_then(|captures| captures.get(1))
        .and_then(|code| code.as_str().parse().ok())
}

fn lora_error_description(ec: i32) -> &'static str {
    match ec {
        -1 => "Parameter is invalid",
        -10 => "Command unknown",
        -11 => "Command is in wrong format",
        -12 => "Command is unavailable in current mode",
        -20 => "Too many parameters. LoRaWAN modem support max 15 parameters",
        -21 => "Length of command is too long (exceed 528 bytes)",
        -22 => "Receive end symbol timeout, command must end with <LF>",
        -23 => "Invalid character received",
        -24 => "Either length of command is too long, receive end symbol timeout or invalid character received",
        _ => "Unknown"
    }
}

fn validate_success_response(response: &str) -> Result<()> {
    if let Some(ec) = parse_lora_error(response) {
        let description = lora_error_description(ec);
        let err = eyre!("Received an error response with code {ec} - {description}");
        Err(err)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lora_error() {
        let ec = parse_lora_error("+AT: ERROR(-1)\r\n").unwrap();
        assert_eq!(ec, -1);
    }

    #[test]
    fn test_parse_received_message() {
        let msg1 = parse_received_message("+TEST: LEN:250, RSSI:-106, SNR:10\r\n");
        assert_eq!(msg1.unwrap(), "Signal strength: -106 dBm, Noise level: 10 dB");

        let msg2 = parse_received_message("+TEST: RX 48656C6C6F\r\n");
        assert_eq!(msg2.unwrap(), "Message: Hello");
    }
}
