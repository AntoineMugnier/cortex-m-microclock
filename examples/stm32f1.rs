#![no_main]
#![no_std]
use cortex_m_rt::entry;

use stm32f1::stm32f103;
use cortex_m::peripheral;

use defmt_rtt as _; // global logger
use panic_probe as _;
use cortex_m_microclock::MicroClock;

#[entry]
fn main() -> ! {

    defmt::info!("hello,  world !");
       
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut p = stm32f103::Peripherals::take().unwrap();
    let rcc = &mut p.RCC; 
   
    // Set 8mhz HSE as direct SYSCLK source    
    rcc.cr.write(|w| w
        .hseon().set_bit()
    );

    // Wait for clock to stabilize
    while !rcc.cr.read().hserdy().bit() {/*NOP*/ }

    defmt::info!("Clock init done");

    //Configure main clock
    let mut dcb = cp.DCB;
    let dwt = cp.DWT;
    let sysclk_freq_hz = 8_000_000;
    let microclock = MicroClock::new(&mut dcb, dwt, sysclk_freq_hz);
    loop{}
}
