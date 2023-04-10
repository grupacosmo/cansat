use crate::SdLogger;
use core::convert::Infallible;
use stm32f4xx_hal::{
    gpio, i2c, pac,
    prelude::*,
    serial, spi,
    timer::{monotonic::MonoTimerUs, DelayUs},
};

pub type Monotonic = MonoTimerUs<pac::TIM2>;
pub type Delay = DelayUs<pac::TIM3>;
pub type Led = gpio::PC13<gpio::Output>;
pub type Gps = cansat_gps::Gps<Serial1>;
pub type SdmmcController =
    embedded_sdmmc::Controller<BlockSpi2, DummyClock, MAX_OPEN_DIRS, MAX_OPEN_FILES>;
pub type SdmmcError = embedded_sdmmc::Error<embedded_sdmmc::SdMmcError>;
pub type Lis3dh = lis3dh::Lis3dh<lis3dh::Lis3dhI2C<I2c1Proxy>>;
pub type Lis3dhError = lis3dh::Error<i2c::Error, Infallible>;
pub type Bme280 = bme280::i2c::BME280<I2c1Proxy>;
pub type Bme280Error = bme280::Error<i2c::Error>;

type BlockSpi2 = embedded_sdmmc::BlockSpi<'static, Spi2, Cs2>;
const MAX_OPEN_DIRS: usize = 4;
const MAX_OPEN_FILES: usize = 4;

type I2c1Proxy = shared_bus::I2cProxy<'static, shared_bus::AtomicCheckMutex<I2c1>>;
type I2c1 = i2c::I2c1<(Scl1, Sda1)>;
type Scl1 = gpio::PB8<gpio::Alternate<4, gpio::OpenDrain>>;
type Sda1 = gpio::PB9<gpio::Alternate<4, gpio::OpenDrain>>;

type Serial1 = serial::Serial1<(Tx1, Rx1)>;
type Tx1 = gpio::PB6<gpio::Alternate<7>>;
type Rx1 = gpio::PB7<gpio::Alternate<7>>;

type Spi2Device = embedded_sdmmc::SdMmcSpi<Spi2, Cs2>;
type Spi2 = spi::Spi2<(Sck2, Miso2, Mosi2)>;
type Cs2 = gpio::PB12<gpio::Output>;
type Sck2 = gpio::PB13<gpio::Alternate<5>>;
type Miso2 = gpio::PB14<gpio::Alternate<5>>;
type Mosi2 = gpio::PB15<gpio::Alternate<5>>;

pub struct CanSat {
    pub monotonic: Monotonic,
    pub delay: Delay,
    pub led: Led,
    pub gps: Gps,
    pub sd_logger: SdLogger,
    pub tracker: accelerometer::Tracker,
    pub i2c1_devices: I2c1Devices,
}

pub struct I2c1Devices {
    pub bme280: Bme280,
    pub lis3dh: Lis3dh,
}

pub struct Board {
    pub monotonic: Monotonic,
    pub delay: Delay,
    pub led: Led,
    pub i2c1: I2c1,
    pub serial1: Serial1,
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

#[derive(Debug, derive_more::From)]
pub enum Error {
    Bme280(Bme280Error),
    Lis3dh(Lis3dhError),
    SdLogger(SdmmcError),
}

impl defmt::Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        use defmt::{write, Debug2Format};
        match self {
            Error::Bme280(e) => write!(fmt, "Failed to initialize BME280: {}", e),
            Error::Lis3dh(e) => write!(fmt, "Failed to initialize LIS3DH: {}", Debug2Format(e)),
            Error::SdLogger(e) => write!(fmt, "Failed to initialize SD logger: {}", e),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn init_drivers(mut board: Board, statik: &'static mut Statik) -> Result<CanSat> {
    let i2c1 = board.i2c1;
    let i2c1_manager = shared_bus::new_atomic_check!(I2c1 = i2c1).unwrap();

    defmt::info!("Initializing sd logger");
    let mut sd_logger = {
        let controller = {
            statik.spi2_device = Some(embedded_sdmmc::SdMmcSpi::new(board.spi2, board.cs2));
            let block_spi2 = statik
                .spi2_device
                .as_mut()
                .unwrap()
                .acquire()
                .map_err(embedded_sdmmc::Error::DeviceError)?;
            SdmmcController::new(block_spi2, DummyClock)
        };

        SdLogger::new(controller)?
    };
    sd_logger.write(b"[NEW RUN]\n")?;

    defmt::info!("Initializing BME280");
    let bme280 = {
        let mut bme280 = Bme280::new_primary(i2c1_manager.acquire_i2c());
        bme280.init(&mut board.delay)?;
        bme280
    };

    defmt::info!("Initializing GPS");
    let gps = {
        board.serial1.listen(serial::Event::Rxne);
        Gps::new(board.serial1)
    };

    defmt::info!("Initializing LIS3DH");
    let mut lis3dh = Lis3dh::new_i2c(i2c1_manager.acquire_i2c(), lis3dh::SlaveAddr::Default)?;
    lis3dh.set_range(lis3dh::Range::G8)?;

    let tracker = accelerometer::Tracker::new(3700.0);
    let i2c1_devices = I2c1Devices { bme280, lis3dh };

    Ok(CanSat {
        monotonic: board.monotonic,
        delay: board.delay,
        led: board.led,
        gps,
        sd_logger,
        tracker,
        i2c1_devices,
    })
}

pub fn init_board(device: pac::Peripherals) -> Board {
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
    let monotonic = device.TIM2.monotonic_us(&clocks);
    let delay = device.TIM3.delay_us(&clocks);

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

    Board {
        monotonic,
        delay,
        led,
        i2c1,
        serial1,
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
