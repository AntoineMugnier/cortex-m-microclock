#![no_main]
#![no_std]
use cortex_m_rt::{entry, exception};

use stm32f1::stm32f103;
use cortex_m::peripheral;

use defmt_rtt as _; // global logger
use panic_probe as _;
use cortex_m_microclock::{CYCCNTClock, Microclock};

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

    const SYSCLK_FREQ_HZ: u32 = 8_000_000;

    let mut microclock = CYCCNTClock::<SYSCLK_FREQ_HZ>::new(&mut dcb, dwt);
    microclock.start();
    let duration = <CYCCNTClock::<SYSCLK_FREQ_HZ> as Microclock>::Duration::secs(1);
    while true {
        let init_inst = microclock.now();
        microclock.delay(duration);
        let t = microclock.now() - init_inst;
        defmt::info!("time_us {}", t.to_micros());
    } 
    defmt::info!("Start");
    microclock.delay(duration);
    defmt::info!("End");

    loop{}
}

#[exception]
fn SysTick(){

}
