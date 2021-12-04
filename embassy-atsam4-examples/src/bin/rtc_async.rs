#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]

#[path = "../example_common.rs"]
mod example_common;
use example_common::*;

use defmt::panic;
use embassy;
use embassy::executor::Spawner;
use embassy::interrupt;
use embassy::time::{Duration, Timer};
use embassy_atsam4;
use embassy_atsam4::hal;
use embassy_atsam4::hal::clock::*;
use embassy_atsam4::hal::rtt::*;
use embassy_atsam4::hal::watchdog;
use embassy_atsam4::pac::{CorePeripherals, Peripherals};

#[embassy::task]
async fn run1() {
    loop {
        info!("BIG INFREQUENT TICK");
        Timer::after(Duration::from_ticks(32768 * 2 as u64)).await;
    }
}

#[embassy::task]
async fn run2() {
    loop {
        info!("tick");
        Timer::after(Duration::from_ticks(13000 as u64)).await;
    }
}

// #[embassy::main(use_hse = 16)]
// async fn main(spawner: Spawner) {
//     let (dp, clocks) = embassy_atsam4::Peripherals::take().unwrap();

//     spawner.spawn(run1()).unwrap();
// }

#[entry]
fn main() -> ! {
    let core = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let clocks = ClockController::new(
        peripherals.PMC,
        &peripherals.SUPC,
        &peripherals.EFC,
        MainClock::RcOscillator12Mhz,
        SlowClock::RcOscillator32Khz,
    );

    Watchdog::new(peripherals.WDT).disable();
    let rtt = RealTimeTimer::new(peripherals.RTT, 20, false);

    let mut rtc = rtc::RTC::new(dp.TIM3, interrupt::take!(TIM3), clocks);
    let rtc = unsafe { make_static(&mut rtc) };
    rtc.start();
    let mut alarm = rtc.alarm1();

    unsafe { embassy::time::set_clock(rtc) };

    let alarm = unsafe { make_static(&mut alarm) };
    executor.set_alarm(alarm);

    loop {}
}
