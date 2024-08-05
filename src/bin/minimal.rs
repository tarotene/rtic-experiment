#![no_main]
#![no_std]

use test_app as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32h7xx_hal::pac, dispatchers = [FDCAN1_IT0])]
mod app {
    use rtic_monotonics::systick::prelude::*;

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

        Mono::start(cx.core.SYST, 36_000_000);

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
        defmt::info!("Hello from task1!");
    }
}
