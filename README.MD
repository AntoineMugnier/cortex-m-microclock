# cortex-m-microclock

This crate provides a software clock which relies on the CYCCNT counter present in most Cortex-M chips
to allow user to measure time and produce delays. The precision of the
clock depends on your microcontroller core frequency. If you have a core running at at least
1MHZ, you will have a microsecond precision.

***Note 0***: Some Cortex-M cores like M0, M0+ and M23 cores does not have a CYCCNT counter.
Therefore, the present crate cannot be used on these chips
***Note 1*** The present crate does not work on multicore systems.

## Underlaying hardware
The clock is based on the CYCCNT counter from the Cortex-M DWT peripheral, which increments
with each processor clock cycle. However as the CYCCNT upcounter is only 32 bits wide, it may overflow
quite rapidly depending on your SYSCLK frequency. The `CYCCNTClock` keeps track of
multiple CYCCNT cycles  using an internal counter so it can be used to evaluate very large durations of time.

## Crate structure
The `CYCCNTClock` is the structure representing the software clock. This structure is a
singleton exposing all the methods of the crate available to user. All these methods are static and
can be called from any thread without concurrency issues.

## How to use this crate
In order to use the clock you should first call the [`CYCCNTClock::init()`] method which takes ownership of
the DWT peripheral. From this point you can use [`CYCCNTClock::now()`] and [`CYCCNTClock::delay()`] methods.
The [`CYCCNTClock::update()`] method should be called periodically to avoid missing
the CYCCNT wrapping around. Use the following equation to find the minimum update frequency:

```min_update_freq = SYS_CLK_FREQ/(2³²)```

Note that the [`CYCCNTClock::now()`] and [`CYCCNTClock::delay()]` methods implicitely call the [`CYCCNTClock::update()`] method.

## Example
 A complete example of the use of this crate on a STMF103 blue pill is featured in the `examples` directory of this
 project.

## Credits
 Many thanks to the authors of `fugit` and `rtic::dwt_systick_monotonic` crates

