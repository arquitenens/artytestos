#![no_std]
#![allow(static_mut_refs)]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

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
use crate::{alloc, bits, vec, THREAD_HELPER};
use crate::alignment::{Alignment, AlignmentEnum};
use crate::alloc::{packed_heap_usable_size, Allocator, KiB, BYTE, HEAP_BLOCKS, STALLOC};
use crate::thread::HelperThread;
use crate::vec::{Vec};



pub fn run(peripherals: Peripherals) -> ! {
    let aligned_heap_start = unsafe {alloc::ALIGNED_HEAP_START};
    let aligned_heap_end = unsafe {alloc::ALIGNED_HEAP_END};
    let packed_heap_start = unsafe {alloc::PACKED_HEAP_START};
    let packed_heap_end = unsafe {alloc::PACKED_HEAP_END};
    
    
    let mut led = gpio::Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default());

    
    let thread = unsafe {&mut THREAD_HELPER};
    println!("thread: {:?}", unsafe {&thread.assume_init_mut().task});
    
    let mut vec: Vec<i32> = Vec::with_capacity(50);
    println!("before bc: {:?}", vec);
    
    /*
    for i in 0..50{
        vec.push(i);
        vec[i as usize] = 67;
        println!("vec: {:?}", vec[i as usize]);
    }
   
     */
    
    //let vec1: InnerVec<u32> = vec::InnerVec::new(alloc::StandardAllocator);
    //info!("Aligned_heap_start: {:?}", aligned_heap_start);
    //info!("aligned_heap_end: {:?}", aligned_heap_end);
    //info!("packed_heap_start: {:?}", packed_heap_start);
    //info!("packed_heap_size {:?}", packed_heap_usable_size());
    //info!("packed_heap_end: {:?}", packed_heap_end);
    //info!("BLOCKS: {:?}", unsafe {alloc::HEAP_BLOCKS});
    
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