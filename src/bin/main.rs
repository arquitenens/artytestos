#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::mem::MaybeUninit;
use esp_hal::peripherals::Peripherals;
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
use lib::thread::HelperThread;
use lib::thread::ThreadState::Uninit;

const MULTICORE: bool = true;


fn init_device() -> esp_hal::peripherals::Peripherals{
    esp_bootloader_esp_idf::esp_app_desc!();
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_240MHz);
    return esp_hal::init(config)
}


#[main]
fn main() -> ! {
    unsafe {lib::alloc::init_heap()};
    let p = init_device();
    let cpu = unsafe {Peripherals::steal()};
    unsafe {
        lib::THREAD_HELPER = MaybeUninit::new(HelperThread::init_thread(cpu.CPU_CTRL));
    }
    lib::code_sec::run(p);
}
