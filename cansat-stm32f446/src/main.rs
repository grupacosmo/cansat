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
        gpio::*,
        pac::{self, USART3},
        prelude::*,
        serial,
        timer::monotonic::fugit::Duration,
        timer::monotonic::MonoTimerUs,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        gps_rx: serial::Rx<USART3>,
        gps_buf_prod: Producer<'static, u8, 512>,
        gps_buf_cons: Consumer<'static, u8, 512>,
        led: Pin<'A', 5, Output>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[init(local = [gps_buf: Queue<u8, 512> = Queue::new()])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("Initializing");

        let device = ctx.device;
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
        let mono = device.TIM2.monotonic_us(&clocks);

        let gpioc = device.GPIOC.split();
        let gpioa = device.GPIOA.split();
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
        let led = gpioa.pa5.into_push_pull_output();
        let local = Local {
            gps_rx,
            gps_buf_prod,
            gps_buf_cons,
            led,
        };
        let shared = Shared {};
        blink::spawn_after(Duration::<u32, 1, 1000000>::from_ticks(0)).unwrap();
        let monotonics = init::Monotonics(mono);
        (shared, local, monotonics)
    }

    #[idle(local = [gps_buf_cons])]
    fn idle(ctx: idle::Context) -> ! {
        defmt::info!("Started idle task");
        let gps_buf_cons = ctx.local.gps_buf_cons;

        let mut msg_buf: Vec<u8, 128> = Vec::new();
        loop {
            if let Some(b) = gps_buf_cons.dequeue() {
                msg_buf.push(b).expect("Message buffer overflow");

                if b == b'\n' {
                    defmt::info!("{=[u8]:a}", &msg_buf);
                    msg_buf.clear();
                }
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

    #[task(local = [led])]
    fn blink(ctx: blink::Context) {
        // toggle led every sec
        let led = ctx.local.led;
        led.toggle();
        defmt::info!("Blink");
        blink::spawn_after(Duration::<u32, 1, 1000000>::from_ticks(1000000)).unwrap();
    }
}
