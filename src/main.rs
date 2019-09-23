#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::{iprintln}; //, Peripherals};
use cortex_m_rt::entry;
use stm32f4::stm32f411;

#[entry]
fn main() -> ! {
    let peripherals = stm32f411::Peripherals::take().unwrap();
    let mut cp = stm32f411::CorePeripherals::take().unwrap();
    let stim = &mut cp.ITM.stim[0];
    iprintln!(stim, "Stellarbourne Broadcast System");

    // Power up the AHB1 peripherals where the GPIO are connected
    peripherals.RCC.ahb1enr.write(|w| w.gpioaen().set_bit());

    // Configure PA5 as push-pull output pin.
    // LED LD2 on Nucleo-F411RE dev board.
    peripherals.GPIOA.otyper.write(|w| w.ot5().push_pull());
    peripherals.GPIOA.moder.write(|w| w.moder5().output());
    peripherals.GPIOA.pupdr.write(|w| w.pupdr5().floating());

    peripherals.GPIOA.odr.write(|w| w.odr5().high());

    loop {}
}
