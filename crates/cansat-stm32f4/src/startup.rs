use crate::{error::Error, SdLogger};
use cansat_lora::ResponseContent;
use core::convert::Infallible;
use stm32f4xx_hal::{
    gpio,
    i2c::{self, I2c1},
    pac,
    prelude::*,
    serial::{self, Serial1, Serial6},
    spi::{self, Spi2},
    timer::DelayUs,
};
use tap::prelude::*;

pub type Delay = DelayUs<pac::TIM3>;
pub type Led = gpio::PC13<gpio::Output>;
pub type Gps = cansat_gps::Gps<Serial1, 256>;
pub type GpsError = cansat_gps::Error<serial::Error>;
pub type Lora = cansat_lora::Lora<Serial6>;
pub type SdmmcController =
    embedded_sdmmc::Controller<BlockSpi2, DummyClock, MAX_OPEN_DIRS, MAX_OPEN_FILES>;
pub type SdmmcError = embedded_sdmmc::Error<embedded_sdmmc::SdMmcError>;
pub type Lis3dh = lis3dh::Lis3dh<lis3dh::Lis3dhI2C<I2c1Proxy>>;
pub type Lis3dhError = lis3dh::Error<i2c::Error, Infallible>;
pub type Bme280 = bme280::i2c::BME280<I2c1Proxy>;
pub type Bme280Error = bme280::Error<i2c::Error>;
pub type LoraError = cansat_lora::Error<serial::Error>;

type BlockSpi2 = embedded_sdmmc::BlockSpi<'static, Spi2, Cs2>;
type Spi2Device = embedded_sdmmc::SdMmcSpi<Spi2, Cs2>;
type Cs2 = gpio::PB12<gpio::Output>;
const MAX_OPEN_DIRS: usize = 4;
const MAX_OPEN_FILES: usize = 4;
type I2c1Proxy = shared_bus::I2cProxy<'static, shared_bus::AtomicCheckMutex<I2c1>>;

pub struct CanSat {
    pub delay: Delay,
    pub led: Led,
    pub gps: Gps,
    pub lora: Option<Lora>,
    pub sd_logger: Option<SdLogger>,
    pub tracker: accelerometer::Tracker,
    pub i2c1_devices: I2c1Devices,
}

pub struct I2c1Devices {
    pub bme280: Option<Bme280>,
    pub lis3dh: Option<Lis3dh>,
}

pub struct Board {
    pub delay: Delay,
    pub led: Led,
    pub i2c1: I2c1,
    pub serial1: Serial1,
    pub serial6: Serial6,
    pub spi2: Spi2,
    pub cs2: Cs2,
}

/// Static memory needed for startup.
///
/// It's named `Statik` because `static` is a reserved keyword.
pub struct Statik {
    spi2_device: Option<Spi2Device>,
}

impl Statik {
    pub const fn new() -> Self {
        Self { spi2_device: None }
    }
}

pub fn init_drivers(mut board: Board, statik: &'static mut Statik) -> Result<CanSat, Error> {
    let i2c1 = board.i2c1;
    let shared_i2c1 = shared_bus::new_atomic_check!(I2c1 = i2c1).unwrap();

    let sd_logger = init_sd_logger(board.spi2, board.cs2, statik)
        .tap_err(|e| defmt::error!("Failed to initialize SD logger: {}", e))
        .ok();

    let lora = init_lora(board.serial6)
        .tap_err(|e| defmt::error!("Failed to initialize Lora: {}", e))
        .ok();

    if sd_logger.is_none() && lora.is_none() {
        return Err(Error::CriticalDevice);
    }

    let bme280 = init_bme280(shared_i2c1.acquire_i2c(), &mut board.delay)
        .tap_err(|e| defmt::error!("Failed to initialize BME280: {}", e))
        .ok();

    let lis3dh = init_lis3dh(shared_i2c1.acquire_i2c())
        .tap_err(|e| defmt::error!("Failed to initialize LIS3DH: {}", defmt::Debug2Format(&e)))
        .ok();
    let tracker = accelerometer::Tracker::new(3700.0);

    let gps = init_gps(board.serial1).map_err(|e| {
        defmt::error!("Failed to initialize GPS: {}", defmt::Debug2Format(&e));
        Error::CriticalDevice
    })?;

    Ok(CanSat {
        delay: board.delay,
        led: board.led,
        gps,
        lora,
        sd_logger,
        tracker,
        i2c1_devices: I2c1Devices { bme280, lis3dh },
    })
}

fn init_sd_logger(spi: Spi2, cs: Cs2, statik: &'static mut Statik) -> Result<SdLogger, SdmmcError> {
    defmt::info!("Initializing sd logger");

    statik.spi2_device = Some(embedded_sdmmc::SdMmcSpi::new(spi, cs));
    let spi_device = statik.spi2_device.as_mut().unwrap();
    let block_spi = spi_device.acquire().map_err(SdmmcError::DeviceError)?;
    let controller = SdmmcController::new(block_spi, DummyClock);
    let mut logger = SdLogger::new(controller)?;
    // controller does some long initialization on first write
    logger.write(b"\n")?;
    Ok(logger)
}

fn init_lora(serial6: Serial6) -> Result<Lora, Error> {
    defmt::info!("Initializing LORA");

    let mut lora = Lora::new(serial6);
    let commands: &[&[u8]] = &[b"AT+MODE=TEST\r\n", b"AT+UART=TIMEOUT,4000\r\n"];

    for cmd in commands {
        let mut response: [u8; 64] = [0; 64];

        lora.send(cmd)?;
        let nread = lora.receive(&mut response)?;

        let response = cansat_lora::parse_response(&response[..nread]).map_err(LoraError::Parse)?;

        if let ResponseContent::Error(ec) = response.content {
            return Err(Error::Response(ec));
        }
    }

    Ok(lora)
}

fn init_bme280(i2c: I2c1Proxy, delay: &mut Delay) -> Result<Bme280, Bme280Error> {
    defmt::info!("Initializing BME280");

    let mut bme280 = Bme280::new_primary(i2c);
    bme280.init(delay)?;
    Ok(bme280)
}

fn init_gps(mut serial: Serial1) -> Result<Gps, GpsError> {
    defmt::info!("Initializing GPS");

    serial.listen(serial::Event::Rxne);
    let mut gps = Gps::new(serial);

    let set_gll_output_rate = b"$PUBX,40,GLL,0,0,0,0,0,0*5C\r\n";
    gps.send(set_gll_output_rate)?;

    let set_gsa_output_rate = b"$PUBX,40,GSA,0,0,0,0,0,0*4E\r\n";
    gps.send(set_gsa_output_rate)?;

    let set_gsv_output_rate = b"$PUBX,40,GSV,0,0,0,0,0,0*59\r\n";
    gps.send(set_gsv_output_rate)?;

    let set_gga_output_rate = b"$PUBX,40,GGA,0,1,0,0,0,0*5B\r\n";
    gps.send(set_gga_output_rate)?;

    let set_vtg_output_rate = b"$PUBX,40,VTG,0,0,0,0,0,0*5E\r\n";
    gps.send(set_vtg_output_rate)?;

    let set_rmc_output_rate = b"$PUBX,40,RMC,0,0,0,0,0,0*47\r\n";
    gps.send(set_rmc_output_rate)?;

    Ok(gps)
}

fn init_lis3dh(i2c: I2c1Proxy) -> Result<Lis3dh, Lis3dhError> {
    defmt::info!("Initializing LIS3DH");

    let mut lis3dh = Lis3dh::new_i2c(i2c, lis3dh::SlaveAddr::Default)?;
    lis3dh.set_range(lis3dh::Range::G8)?;
    Ok(lis3dh)
}

pub fn init_board(device: pac::Peripherals) -> Board {
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
    let delay = device.TIM3.delay_us(&clocks);

    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();
    let gpioc = device.GPIOC.split();

    let led = gpioc.pc13.into_push_pull_output();

    let i2c1 = {
        let scl1 = gpiob
            .pb8
            .into_alternate()
            .internal_pull_up(false)
            .set_open_drain();
        let sda1 = gpiob
            .pb9
            .into_alternate()
            .internal_pull_up(false)
            .set_open_drain();
        let mode = i2c::Mode::Fast {
            frequency: 400000.Hz(),
            duty_cycle: i2c::DutyCycle::Ratio2to1,
        };
        device.I2C1.i2c((scl1, sda1), mode, &clocks)
    };

    let spi2 = {
        let sck2 = gpiob.pb13.into_alternate();
        let miso2 = gpiob.pb14.into_alternate();
        let mosi2 = gpiob.pb15.into_alternate();
        let mode = spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        };
        device
            .SPI2
            .spi((sck2, miso2, mosi2), mode, 400000.Hz(), &clocks)
    };
    let cs2 = gpiob.pb12.into_push_pull_output();

    let serial1 = {
        let tx1 = gpiob.pb6.into_alternate();
        let rx1 = gpiob.pb7.into_alternate();
        let config = serial::Config::default().baudrate(9600.bps());
        device
            .USART1
            .serial((tx1, rx1), config, &clocks)
            .expect("Invalid USART1 config")
    };

    let serial6 = {
        let tx6 = gpioa.pa11.into_alternate();
        let rx6 = gpioa.pa12.into_alternate();
        let config = serial::Config::default().baudrate(9600.bps());
        device
            .USART6
            .serial((tx6, rx6), config, &clocks)
            .expect("Invalid USART6 config")
    };

    Board {
        delay,
        led,
        i2c1,
        serial1,
        serial6,
        spi2,
        cs2,
    }
}

pub struct DummyClock;

impl embedded_sdmmc::TimeSource for DummyClock {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
