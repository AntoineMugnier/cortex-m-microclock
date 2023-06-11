#![no_main]
#![no_std]
use kaori_hsm::*;
use cortex_m_rt::entry;
use hsm::{BasicStateMachine, BasicEvt};
mod hsm;

use stm32f1::stm32f103;
use cortex_m::peripheral;

use defmt_rtt as _; // global logger
use panic_probe as _;

#[entry]
fn main() -> ! {

    defmt::println!("hello,  world !");
   
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut peripherals = stm32f103::Peripherals::take().unwrap();

    //Configure main clock
    let mut rcc = peripherals.RCC;
    let tim2 = &peripherals.TIM2;

    let basic_state_machine = BasicStateMachine::new();
    
    let mut sm = StateMachine::from(basic_state_machine);

    sm.init();

    let evt_list = [BasicEvt::A, BasicEvt::B];

    for evt in evt_list {
        sm.dispatch(&evt);
    }

    loop{}
}
