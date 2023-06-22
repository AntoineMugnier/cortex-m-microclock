#![no_main]
#![no_std]
use cortex_m_rt::{entry, exception};

use stm32f1::stm32f103;
use cortex_m::peripheral::syst;

use defmt_rtt as _; // global logger
use panic_probe as _;
use cortex_m_microclock::{CYCCNTClock, Instant, Duration};
use cortex_m::interrupt::{self, Mutex};
use core::{cell::RefCell, ops::DerefMut};

const SYSCLK_FREQ_HZ : u32 = 8_000_000;
static MICROCLOCK: Mutex<RefCell<Option<CYCCNTClock::<SYSCLK_FREQ_HZ>>>> =
    Mutex::new(RefCell::new(None));

fn micro_now() -> Instant<SYSCLK_FREQ_HZ>{
   return interrupt::free(|cs|{
   let  mut microclock =  MICROCLOCK.borrow(cs).borrow_mut();
        return microclock.deref_mut().as_mut().unwrap().now();
    }
    );
    
}

#[entry]
fn main() -> ! {

    defmt::info!("hello,  world !");
       
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut p = stm32f103::Peripherals::take().unwrap();


    let mut systick = cp.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();
    systick.enable_interrupt();

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
    interrupt::free(|cs| MICROCLOCK.borrow(cs).replace(Some(microclock)));

    let duration = Duration::<SYSCLK_FREQ_HZ>::secs(1);

    while true {
        let init_inst = micro_now();
        let t = micro_now() - init_inst;
        defmt::info!("time_us {}", t.to_micros());
    }

    loop{}
}

#[exception]
fn SysTick(){
   interrupt::free(|cs|
   if let Some(ref mut microclock) =  *MICROCLOCK.borrow(cs).borrow_mut(){
            microclock.update();
    });
}
