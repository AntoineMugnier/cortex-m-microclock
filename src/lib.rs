#![no_std]

use cortex_m::peripheral::{DCB, DWT};

pub type Instant<const TIMER_HZ: u32> = fugit::TimerInstantU64<TIMER_HZ>;
pub type Duration<const TIMER_HZ: u32> = fugit::TimerDurationU64<TIMER_HZ>;
 
pub struct CYCCNTClock<const SYSCLK_HZ: u32>{
    dwt: DWT,
    previous_cyccnt_val : u32,
    nb_cyccnt_cycles : u32,
}

impl <const SYSCLK_HZ :u32 >Drop for  CYCCNTClock<SYSCLK_HZ>{
    fn drop(&mut self) {
        self.dwt.disable_cycle_counter();
    }
}

impl <const SYSCLK_HZ :u32 > CYCCNTClock<SYSCLK_HZ>{

   const MAX_CYCCNT_VAL: u32 = core::u32::MAX;
   
    pub fn now(&mut self) -> Instant<SYSCLK_HZ> {
        
        //Call `update()` because the CYCCNT counter could have wrapped around since the last time
        //`update()` was called
        self.update();

        let acc_cyccnt_val : u64 = (self.nb_cyccnt_cycles as u64) *(Self::MAX_CYCCNT_VAL as u64 +1) + (self.dwt.cyccnt.read() as u64);
        Instant::from_ticks(acc_cyccnt_val)
    }

    pub fn delay(&mut self, duration: Duration<SYSCLK_HZ>) {
        let instant_init = self.now();
        while (self.now() - instant_init) < duration{
            // NOP
        }
    }

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
        dwt.enable_cycle_counter();
        dwt.set_cycle_count(0);

        CYCCNTClock{
            nb_cyccnt_cycles: 0,
            previous_cyccnt_val: 0,
            dwt
        }

}

}

