#![feature(ptr_internals)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use esp_hal::peripherals::CPU_CTRL;
use esp_hal::system::CpuControl;
use core::mem::MaybeUninit;
use crate::thread::HelperThread;

pub static mut APP_CORE_INIT: MaybeUninit<CpuControl<'_>> = MaybeUninit::uninit();

pub mod alignment;

pub mod vec;
pub mod code_sec;
pub mod panic;
pub mod alloc;
pub mod bits;
pub mod borrow;
pub mod thread;

