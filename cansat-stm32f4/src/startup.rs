use crate::{
    error::{Report, WrapErr},
    Spi2Device,
};
use stm32f4xx_hal::{i2c, pac, prelude::*, serial, spi};

pub struct CanSat {
    pub monotonic: crate::Monotonic,
    pub delay: crate::Delay,
    pub led: crate::Led,
    pub bme280: crate::Bme280,
    pub gps: crate::Gps,
    pub sd_logger: crate::SdLogger,
}

pub struct Board {
    pub monotonic: crate::Monotonic,
    pub delay: crate::Delay,
    pub led: crate::Led,
    pub i2c1: crate::I2c1,
    pub serial1: crate::Serial1,
    pub spi2: crate::Spi2,
    pub cs2: crate::Cs2,
}

pub fn init_drivers(
    mut board: Board,
    spi2_device: &'static mut Option<Spi2Device>,
) -> Result<CanSat, Report> {
    defmt::info!("Initializing drivers");

    let mut sd_logger = {
        let controller = {
            *spi2_device = Some(embedded_sdmmc::SdMmcSpi::new(board.spi2, board.cs2));
            let block_spi2 = spi2_device
                .as_mut()
                .unwrap()
                .acquire()
                .wrap_err("Failed to acquire block spi")?;
            crate::SdmmcController::new(block_spi2, crate::DummyClock)
        };

        crate::sd_logger::SdLogger::new(controller).wrap_err("Failed to initialize SdLogger")?
    };

    let _ = sd_logger.write(b"[NEW RUN]");

    let mut bme280 = crate::Bme280::new_primary(board.i2c1);
    bme280
        .init(&mut board.delay)
        .wrap_err("Failed to initialize BME280")?;

    let gps = {
        board.serial1.listen(serial::Event::Rxne);
        crate::Gps::new(board.serial1)
    };

    Ok(CanSat {
        monotonic: board.monotonic,
        delay: board.delay,
        led: board.led,
        bme280,
        gps,
        sd_logger,
    })
}

pub fn init_board(device: pac::Peripherals) -> Board {
    defmt::info!("Initializing the board");

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
