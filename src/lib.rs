#![no_std]

use cortex_m::peripheral::{DCB, DWT};

pub struct CYCCNTClock<const SYSCLK_HZ: u32>{
    dwt: DWT,
    previous_cyccnt_val : u32,
    nb_cyccnt_cycles : u32,
}


pub trait Microclock{
    type Instant;
    type Duration;
    
    fn start(&mut self);
    fn stop(&mut self);
    fn reset(&mut self);
    fn now(&mut self) -> Self::Instant;
    fn delay(&mut self, duration: Self::Duration);
}

impl <const TIMER_HZ: u32> Microclock for CYCCNTClock<TIMER_HZ>{ 
    type Instant = fugit::TimerInstantU64<TIMER_HZ>;
    type Duration = fugit::TimerDurationU64<TIMER_HZ>;
    
    fn start(&mut self){
        self.reset();
        self.dwt.enable_cycle_counter();
    }
    
    fn stop(&mut self) {
        self.dwt.disable_cycle_counter();
    }

    fn now(&mut self) -> Self::Instant {
        let acc_cyccnt_val : u64 = (self.nb_cyccnt_cycles as u64) *(Self::MAX_CYCCNT_VAL as u64 +1) + (self.dwt.cyccnt.read() as u64);
        Self::Instant::from_ticks(acc_cyccnt_val)
    }

    fn reset(&mut self){
       self.dwt.set_cycle_count(0);
       self.previous_cyccnt_val = 0;
       self.nb_cyccnt_cycles = 0;
    }

    fn delay(&mut self, duration: Self::Duration) {
        let instant_init = self.now();
        while (self.now() - instant_init) < duration{
            // NOP
        }
    }
}

impl <const SYSCLK_HZ :u32 > CYCCNTClock<SYSCLK_HZ>{
   const MAX_CYCCNT_VAL: u32 = core::u32::MAX;

   pub fn update(&mut self){
        
        let cyccnt_val = self.dwt.cyccnt.read();

        // increment the number of counted CYCCNT cycles in case of CYCCNT counter overflow
        if self.previous_cyccnt_val > cyccnt_val {
            self.nb_cyccnt_cycles+=1;
        }

        self.previous_cyccnt_val = cyccnt_val;
    }

    pub fn new(dcb: &mut DCB, mut dwt : DWT) -> CYCCNTClock<SYSCLK_HZ>{

        dcb.enable_trace();
        DWT::unlock();
        assert!(DWT::has_cycle_counter());

        dwt.set_cycle_count(0);

        CYCCNTClock{
            nb_cyccnt_cycles: 0,
            previous_cyccnt_val: 0,
            dwt
        }

}

}

