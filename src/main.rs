#![no_main]
#![no_std]

use panic_itm as _; // panic handler

#[rtic::app(device = stm32f4::stm32f411)]
mod app {
    use cortex_m_semihosting::debug;

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        assert!(cortex_m::Peripherals::take().is_none());
        debug::exit(debug::EXIT_SUCCESS);

        init::LateResources {}
    }
}