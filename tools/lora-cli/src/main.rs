use std::{
    io::{BufRead, BufReader},
    time::Duration,
};

use clap::Parser;
use eyre::WrapErr;
use once_cell::sync::Lazy;
use regex::Regex;
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
    /// Start a receive loop
    Receive(ReceiveArgs),
}

#[derive(Debug, clap::Parser)]
struct ReceiveArgs {
    /// Serial port to open
    #[arg(short, long)]
    port: String,
    /// Baudrate
    #[arg(short, long)]
    baudrate: u32,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();
    match args.cmd {
        Cmd::Ports => list_ports(),
        Cmd::Receive(args) => receive(args),
    }?;
    Ok(())
}

fn list_ports() -> eyre::Result<()> {
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

fn receive(args: ReceiveArgs) -> eyre::Result<()> {
    eprintln!("Configuring...");

    let port = serialport::new(args.port, args.baudrate)
        .timeout(Duration::from_secs(10))
        .open()
        .wrap_err("Failed to open serial port")?;

    let mut lora = Lora::new(port);

    lora.transmit(b"AT+MODE=TEST\r\n")
        .wrap_err("Failed to set test mode")?;

    eprintln!("Listening...");

    for result in lora.listen()? {
        match result {
            Ok(msg) => println!("{msg}"),
            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}

struct Lora {
    port: Box<dyn SerialPort>,
}

impl Lora {
    fn new(port: Box<dyn SerialPort>) -> Self {
        Self { port }
    }

    fn receive(&mut self) -> eyre::Result<String> {
        let mut port = BufReader::new(&mut self.port);
        let mut response = String::new();
        port.read_line(&mut response)
            .wrap_err("Failed to read message")?;
        validate_success_response(&response)?;
        Ok(response)
    }

    fn send(&mut self, input: &[u8]) -> eyre::Result<usize> {
        self.port.write(input).wrap_err("Failed to write message")
    }

    fn transmit(&mut self, input: &[u8]) -> eyre::Result<String> {
        self.send(input)?;
        self.receive()
    }

    fn listen(mut self) -> eyre::Result<impl Iterator<Item = eyre::Result<String>>> {
        self.port
            .set_timeout(Duration::from_secs(0))
            .wrap_err("Failed to disable timeout")?;
        let port = BufReader::new(self.port);

        let iter = port.lines().map(|result| {
            let response = result.wrap_err("Failed to read message")?;
            validate_success_response(&response)?;
            Ok(response)
        });

        Ok(iter)
    }
}

fn parse_lora_error(input: &str) -> Option<i32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"ERROR\((-?\d+)\)").unwrap());
    RE.captures(input)
        .and_then(|captures| captures.get(1))
        .and_then(|code| code.as_str().parse().ok())
}

fn lora_error_description(ec: i32) -> &'static str {
    match ec {
        -1 => "Parameters is invalid",
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

fn validate_success_response(response: &str) -> eyre::Result<()> {
    if let Some(ec) = parse_lora_error(response) {
        let description = lora_error_description(ec);
        let err = eyre::eyre!("Received an error response with code {ec} - {description}");
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
}
