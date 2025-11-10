#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::xtensa_lx::get_stack_pointer;
use esp_hal::clock::CpuClock;
use esp_hal::gpio;
use esp_hal::ram;
use esp_hal::gpio::Level;
use esp_hal::gpio::OutputConfig;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use log::info;

use lib;


#[main]
fn main() -> ! {
    unsafe {lib::alloc::init_heap()};
    lib::code_sec::run();
}
