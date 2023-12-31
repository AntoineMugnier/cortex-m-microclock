// Copyright © 2023 cortex-m-microclock. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This example has been tested on a a STM32F103 blue pill (STM32F103C8T6 MCU) target with Ubuntu
// 22.0.4.  
// The following code initializes the MCU and triggers a loop which repeatidly creates a 1 second delay and sends its measure to the host through RTT tracing.
// 
// ##  How to run it ?
// - You must have installed `probe-run` and the `thumbv7m-none-eabi` toolchain . 
//      - Follow instructions [here](https://crates.io/crates/probe-run) to install probe-run.
//      - For installing the toolchain : `cargo install thumbv7m-none-eabi` 
// - Connect a ST-LINK probe to the blue pill with SWCLK, SWDIO, RST, GND, and 3V3 lines. Be sure
// your udev rules are well configured on Linux.
// - run `DEFMT_LOG=trace cargo run --example stm32f1` at the root of the project. Microcontroller
// flashing should occur and you should see a new trace emitted every second by the microcontroller
// on your console.
//

#![no_main]
#![no_std]

use cortex_m_rt::{entry, exception};

use stm32f1::stm32f103;
use cortex_m::peripheral::syst;

use defmt_rtt as _; // global logger
use panic_probe as _;

const SYSCLK_FREQ_HZ : u32 = 8_000_000;

// Aliases for the library generic types 
type Microclock = cortex_m_microclock::CYCCNTClock<SYSCLK_FREQ_HZ>;
type Duration = cortex_m_microclock::Duration::<SYSCLK_FREQ_HZ>;

#[entry]
fn main() -> ! {
       
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut p = stm32f103::Peripherals::take().unwrap();

    //Configure clock tree
    let rcc = &mut p.RCC;

    // Set 8mhz HSE as direct SYSCLK source    
    rcc.cr.write(|w| w
        .hseon().set_bit()
    );

    // Wait for clock to stabilize
    while !rcc.cr.read().hserdy().bit() {/*NOP*/ }

    // Configure Systick
    // Systick is fed by SYSCLK (8MHz) divided by 8 (No division by AHB Prescaler).
    let mut systick = cp.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(999);
    systick.clear_current();
    systick.enable_counter();
    
    //Setup CYCCNT clock
    let mut dcb = cp.DCB;
    let dwt = cp.DWT;
    Microclock::init(&mut dcb, dwt);
    
    // Delayed enabling of systick IRQs as `Microclock::update()` should not be triggered before call to `Microclock::init()`
    systick.enable_interrupt();

    let duration = Duration::secs(1);
    loop {
        let init_inst = Microclock::now();
        Microclock::delay(duration);
        let elapsed_time = Microclock::now() - init_inst;
        defmt::info!("time_us {}", elapsed_time.to_micros());
    }
}

#[exception]
fn SysTick(){
    Microclock::update();
}
