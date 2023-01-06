#![deny(unsafe_code)]
#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0])]
mod app {
    use cansat::defmt;
    use heapless::{
        spsc::{Consumer, Producer, Queue},
        Vec,
    };
    use stm32f4xx_hal::{
        gpio::{Output, PA4, PA5, PB3, PB4, PB5, PC13},
        pac::{SPI1, TIM2, TIM3, USART3},
        prelude::*,
        serial,
        spi::Spi,
        timer::{monotonic::MonoTimerUs, DelayMs},
    };
    use sx127x_lora::LoRa;
        
    type Sck1 = PB3<Output>;
    type Miso1 = PB4<Output>;
    type Mosi1 = PB5<Output>;
    const BIDI: bool = false;
    type Spi1 = Spi<SPI1, (Sck1, Miso1, Mosi1), BIDI>;
    type Cs = PA4<Output>;
    type Reset = PC13<Output>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        gps_rx: serial::Rx<USART3>,
        gps_buf_prod: Producer<'static, u8, 512>,
        gps_buf_cons: Consumer<'static, u8, 512>,
        led: PA5<Output>,
        lora: LoRa<Spi1, Cs, Reset, DelayMs<TIM3>>
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<TIM2>;

    #[init(local = [gps_buf: Queue<u8, 512> = Queue::new()])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("Initializing");

        let device = ctx.device;
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
        let mono = device.TIM2.monotonic_us(&clocks);

        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();

        let led = gpioa.pa5.into_push_pull_output();

        let usart3_tx_pin = gpioc.pc10.into_alternate();
        let usart3_rx_pin = gpioc.pc11.into_alternate();

        let gps_serial = device
            .USART3
            .serial(
                (usart3_tx_pin, usart3_rx_pin),
                serial::Config::default().baudrate(9600.bps()),
                &clocks,
            )
            .unwrap();
        let (_gps_tx, mut gps_rx) = gps_serial.split();
        gps_rx.listen();

        let gps_buf = ctx.local.gps_buf;
        let (gps_buf_prod, gps_buf_cons) = gps_buf.split();

        let lora = {
            let sck = gpiob.pb3.into_push_pull_output();
            let miso = gpiob.pb4.into_push_pull_output();
            let mosi = gpiob.pb5.into_push_pull_output();
            let cs = gpioa.pa4.into_push_pull_output();
            let reset = gpioc.pc13.into_push_pull_output();

            let spi = device
                .SPI1
                .spi((sck, miso, mosi), sx127x_lora::MODE, 8.MHz(), &clocks);

            let frequency = 915;
            let delay = device.TIM3.delay_ms(&clocks);
            LoRa::new(spi, cs, reset, frequency, delay).unwrap()
        };

        blink::spawn().unwrap();

        let local = Local {
            gps_rx,
            gps_buf_prod,
            gps_buf_cons,
            led,
            lora,
        };
        let shared = Shared {};
        let monotonics = init::Monotonics(mono);
        (shared, local, monotonics)
    }

    #[idle(local = [gps_buf_cons, lora])]
    fn idle(ctx: idle::Context) -> ! {
        defmt::info!("Started idle task");
        let gps_buf_cons = ctx.local.gps_buf_cons;
        let lora = ctx.local.lora;

        let mut msg_buf: Vec<u8, 128> = Vec::new();
        loop {
            if let Some(b) = gps_buf_cons.dequeue() {
                msg_buf.push(b).expect("Message buffer overflow");

                if b == b'\n' {
                    defmt::info!("{=[u8]:a}", &msg_buf);
                    msg_buf.clear();
                }
            }

            let poll = lora.poll_irq(Some(30)); //30 Second timeout
            match poll {
                Ok(size) =>{
                    defmt::info!(
                        "New Packet with size {} and RSSI: {}",
                        size,
                        lora.get_packet_rssi()
                    );
                    let buffer = lora.read_packet().unwrap(); // NOTE: 255 bytes are always returned
                    defmt::info!("{}", buffer);
                },
                Err(_) => defmt::info!("Timeout")
            }
        }
    }

    /// USART3 interrupt handler that reads data into the gps working buffer
    #[task(binds = USART3, local = [gps_rx, gps_buf_prod])]
    fn gps_irq(ctx: gps_irq::Context) {
        let gps_rx = ctx.local.gps_rx;
        let gps_buf_prod = ctx.local.gps_buf_prod;

        let b = gps_rx.read().unwrap();
        gps_buf_prod.enqueue(b).unwrap();
    }

    /// Toggles led every second
    #[task(local = [led])]
    fn blink(ctx: blink::Context) {
        let led = ctx.local.led;
        led.toggle();
        defmt::info!("Blink");
        blink::spawn_after(1.secs()).unwrap();
    }
}
