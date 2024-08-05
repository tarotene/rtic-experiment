#![no_main]
#![no_std]

use test_app as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [FDCAN1_IT0])]
mod app {
    use rtic_monotonics::systick::prelude::*;
    use stm32h7xx_hal::prelude::*;

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
    }

    systick_monotonic!(Mono, 1000);

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let _ = {
            let pwr = cx.device.PWR.constrain();
            let rcc = cx.device.RCC.constrain();

            rcc.sys_ck(200.MHz(/* 200_000_000 Hz */)).freeze(pwr.freeze(), &cx.device.SYSCFG)
        };

        Mono::start(cx.core.SYST, 200_000_000);

        task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    // TODO: Add tasks
    #[task(priority = 1)]
    async fn task1(_cx: task1::Context) {
        loop {
            defmt::info!("Hello from task1!");

            Mono::delay(1000.millis()).await;
        }
    }
}
