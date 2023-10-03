//! This crate provides a software clock which relies on the CYCCNT counter present in most Cortex-M chips 
//! to allow user to measure time and produce delays. The precision of the
//! clock depends on your microcontroller core frequency. If you have a core running at at least
//! 1MHZ, you will have a microsecond precision. 
//! 
//! ***Note 0***: Some Cortex-M cores like M0, M0+ and M23 cores does not have a CYCCNT counter.
//! Therefore, the present crate cannot be used on these chips
//! ***Note 1*** The present crate does not work on multicore systems.
//! 
//! # Underlaying hardware
//! The clock is based on the CYCCNT counter from the Cortex-M DWT peripheral, which increments
//! with each processor clock cycle. However as the CYCCNT upcounter is only 32 bits wide, it may overflow 
//! quite rapidly depending on your SYSCLK frequency. The `CYCCNTClock` keeps track of 
//! multiple CYCCNT cycles  using an internal counter so it can be used to evaluate very large durations of time.
//!
//! # Crate structure
//! The `CYCCNTClock` is the structure representing the software clock. This structure is a
//! singleton exposing all the methods of the crate available to user. All these methods are static and
//! can be called from any thread without concurrency issues.    
//! 
//! # How to use this crate
//! In order to use the clock you should first call the [`CYCCNTClock::init()`] method which takes ownership of
//! the DWT peripheral. From this point you can use [`CYCCNTClock::now()`] and [`CYCCNTClock::delay()`] methods.
//! The [`CYCCNTClock::update()`] method should be called periodically to avoid missing 
//! the CYCCNT wrapping around. Use the following equation to find the minimum update frequency:
//!
//! ```min_update_freq = SYS_CLK_FREQ/(2³²)```
//!
//! Note that the [`CYCCNTClock::now()`] and [`CYCCNTClock::delay()]` methods implicitely call the [`CYCCNTClock::update()`] method.
//! 
//! # Example
//!  A complete example of the use of this crate on a STMF103 blue pill is featured in the `examples` directory of this
//!  project.
//!  
//!# Credits
//!  Many thanks to the authors of `fugit` and `rtic::dwt_systick_monotonic` crates
//!

#![no_std]

use cortex_m::peripheral::{DCB, DWT};
use cortex_m::interrupt::{self, Mutex};
use core::{cell::RefCell, ops::DerefMut};
use cortex_m::asm;

pub type Instant<const TIMER_HZ: u32> = fugit::TimerInstantU64<TIMER_HZ>;
pub type Duration<const TIMER_HZ: u32> = fugit::TimerDurationU64<TIMER_HZ>;

static DWT:Mutex<RefCell<Option<DWT>>> =
    Mutex::new(RefCell::new(None));

static PREVIOUS_CYCCNT_VAL : Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
static NB_CYCCNT_CYCLES : Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));


/// Clock based on the Cortex-M CYCCNT counter allowing to measure time durations and produce
/// delays.  
/// Precise at a microsecond scale if your SYSCLK clock is greater than 1MHz
pub struct CYCCNTClock<const SYSCLK_HZ: u32>{
}

impl <const SYSCLK_HZ :u32 > CYCCNTClock<SYSCLK_HZ>{

   const MAX_CYCCNT_VAL: u32 = core::u32::MAX;

    /// Return an `Instant` object corresponding to a snapshot created at the time this method was called.
    /// Panic if the counter has not been initialized with `init()` before.
    ///
    /// ```
    ///    const SYSCLK_FREQ_HZ : u32 = 8_000_000;
    ///    let t1 = CYCCNTClock<SYSCLK_FREQ_HZ>::now();
    ///     
    ///    // Wait 100 us
    ///    let duration = Duration::micros(100);
    ///    CYCCNTClock<SYSCLK_FREQ_HZ>::delay(duration)
    ///    
    ///    let t2 = CYCCNTClock<SYSCLK_FREQ_HZ>::now();
    ///    let elpased_time = t2 - t1; // Very small elapsed time
    ///    println!("time_us {}", elapsed_time.to_micros());
    /// ```
    pub fn now() -> Instant<SYSCLK_HZ> {
        interrupt::free( |cs|{
          //Call `update()` because the CYCCNT counter could have wrapped around since the last time
          //`update()` was called
          Self::update();
       
          let nb_cyccnt_cycles : u32 = *NB_CYCCNT_CYCLES.borrow(cs).borrow();
          
          if let Some(ref mut dwt) = DWT.borrow(cs).borrow_mut().deref_mut().as_mut(){
              let acc_cyccnt_val : u64 = (nb_cyccnt_cycles as u64) *(Self::MAX_CYCCNT_VAL as u64 +1) + (dwt.cyccnt.read() as u64);
              Instant::from_ticks(acc_cyccnt_val)
          }
          else{
              panic!("Counter not initialized");
          }
        })
    }
        
    /// Blocking wait for the duration specified as argument
    /// Interrupts can still trigger during this call.
    /// Panic if the counter has not been initialized with `init()` before.
    /// ```
    ///    const SYSCLK_FREQ_HZ : u32 = 8_000_000;
    ///    let t1 = CYCCNTClock<SYSCLK_FREQ_HZ>::now();
    ///     
    ///    // Wait 100 us
    ///    let duration = Duration::micros(100);
    ///    CYCCNTClock<SYSCLK_FREQ_HZ>::delay(duration)
    ///    
    ///    let t2 = CYCCNTClock<SYSCLK_FREQ_HZ>::now();
    ///    let elpased_time = t2 - t1; // Very small elapsed time
    ///    println!("time_us {}", elapsed_time.to_micros());
    /// ```
    pub fn delay(duration: Duration<SYSCLK_HZ>) {
        let instant_init = Self::now();
        while (Self::now() - instant_init) < duration{
            asm::nop();
        }
    }
    
    /// Synchronize the hardware counter with this clock.
    /// Must be called at least one time for every CYCCNT counter cycle after init. Otherwise
    /// time counting will be corrupted.
    /// In general, calling this method in every SysTick IRQ call is the simpler option
    /// This method will NOT panic if the `init()` method has not be called first.
   pub fn update(){
        
       interrupt::free( |cs|{
            if let Some(ref mut dwt) = DWT.borrow(cs).borrow_mut().deref_mut().as_mut(){
                let cyccnt_val = dwt.cyccnt.read();
                
                let mut previous_cyccnt_val = PREVIOUS_CYCCNT_VAL.borrow(cs).borrow_mut();
                
                // increment the number of counted CYCCNT cycles in case of CYCCNT counter overflow
                if *previous_cyccnt_val as u32 > cyccnt_val {
                    NB_CYCCNT_CYCLES.borrow(cs).replace_with(|&mut old| old +1);
                }

                *previous_cyccnt_val = cyccnt_val as usize;
            } 
        });
    }
    
    /// Enable CYCCNT counting capability of the cortex core and
    /// start the couting
    ///```
    /// let mut cp = cortex_m::Peripherals::take().unwrap();
    /// let mut dcb = cp.DCB;
    /// let dwt = cp.DWT;
    /// CYCCNTClock<SYSCLK_FREQ_HZ>::init(&mut dcb, dwt);
    ///```
    pub fn init(dcb: &mut DCB, mut dwt : DWT){

       interrupt::free( |cs|{
            assert!(DWT::has_cycle_counter());

            dcb.enable_trace();
            DWT::unlock();
            dwt.enable_cycle_counter();
            dwt.set_cycle_count(0);
            
            // Store the DWT peripheral into a shared object
            DWT.borrow(cs).borrow_mut().replace(dwt); 
        });
}

}

