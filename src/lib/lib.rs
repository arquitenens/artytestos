#![feature(ptr_internals)]
#![no_std]
#![no_main]

pub mod alignment;

pub mod vec;
pub mod code_sec;
pub mod panic;
pub mod alloc;
pub mod bits;
mod borrow;