#![no_std]
#![allow(static_mut_refs)]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::sync::atomic::{AtomicU8, Ordering};
use core::ptr::slice_from_raw_parts;
use crate::alloc;
use core::any::TypeId;
use core::intrinsics::type_id;
use esp_hal::clock::CpuClock;
use esp_hal::gpio;

use esp_hal::system::CpuControl;
use esp_hal::gpio::Level;
use esp_hal::gpio::OutputConfig;
use esp_hal::time::{Duration, Instant};
use esp_hal;
use esp_hal::peripherals::Peripherals;
use esp_println::println;
use log::info;
use crate::thread;
use crate::alignment::{Alignment, AlignmentEnum};
use crate::alloc::{packed_heap_usable_size, Allocator, KiB, BYTE, HEAP_BLOCKS, STALLOC};
use crate::{core_input, thread::{HelperThread, ThreadState, CORE_STATE}};
use crate::vec::{Vec};


pub fn run(peripherals: Peripherals) -> ! {
    let aligned_heap_start = unsafe {alloc::ALIGNED_HEAP_START};
    let aligned_heap_end = unsafe {alloc::ALIGNED_HEAP_END};
    let packed_heap_start = unsafe {alloc::PACKED_HEAP_START};
    let packed_heap_end = unsafe {alloc::PACKED_HEAP_END};
    
    
    let mut led = gpio::Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default());
    
    //println!("Data before: {:?}", unsafe {&thread::CORE_DATA});
    let load: [u32; 10] = [67; 10];
    let cl = core_input!(u32, data, load, {
        println!("t:{:?} ", const {type_id::<i32>()});
            let safe_data = unsafe {&mut *data};
            let x = safe_data;
            x[0] = 50;
            println!("HIII {:?}", x);
        });

    let mut thread = thread::HelperThread::new();
    
    let t_cl = ||{
        println!("Thread started");
        CORE_STATE.store(1, Ordering::Release);
    };
    thread.execute(cl);
    println!("CORE_STATE: {:?}", CORE_STATE.load(Ordering::Acquire));
    println!("Data after: {:?}", unsafe {&thread::CORE_DATA});
    println!("load: {:?}", load);
    //t.state = ThreadState::Active;
    //println!("state: {:?}", t.state);
    let x: Vec<i32> = Vec::new();
    
    loop {
        unsafe {
            
            //info!("LED? {:?}", led.toggle());
            let loop_time_start = Instant::now();
            let loop_end = loop_time_start.elapsed();
            //info!("time per loop: {:?}", loop_end);
            led.toggle();

        }
        let delay_start = Instant::now();    
        while delay_start.elapsed() < Duration::from_millis(1000) {}
    }
}

//1073420144 HEAP START
//1073580144 END

//1073580152 START 
//1073606592 END