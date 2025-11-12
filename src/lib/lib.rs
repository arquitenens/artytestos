#![feature(ptr_internals)]
#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use crate::thread::HelperThread;

pub static mut THREAD_HELPER: MaybeUninit<HelperThread> = MaybeUninit::uninit();

pub mod alignment;

pub mod vec;
pub mod code_sec;
pub mod panic;
pub mod alloc;
pub mod bits;
pub mod borrow;
pub mod thread;