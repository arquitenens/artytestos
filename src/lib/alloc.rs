#![no_std]
#![allow(static_mut_refs)]

use core::cell::RefCell;
use core::ptr::NonNull;
use esp_hal::gpio::OutputSignal::PWM0_0A;
use esp_hal::xtensa_lx::interrupt::free;
use esp_println::logger::init_logger_from_env;
use esp_println::print;
use log::info;
use crate::bits::{read_bit, tag_int, BIT};


unsafe extern "C" {
    static mut _bss_end: u8;
}

const STACK_RESERVED: usize = 0x1000; //4KiB stack reserved buffer
const ALIGNED_HEAP_SIZE: usize = 0x27100;
const SMALL_BUFFER: usize = 8;

const HEAP_BLOCK_AMOUNT: usize = 5000; //change as needed i guess
pub static mut HEAP_BLOCKS: [u8; HEAP_BLOCK_AMOUNT] = [0; HEAP_BLOCK_AMOUNT]; //list of all heap blocks


pub const BYTE: usize = 1;
pub const KiB: usize = BYTE * 1024;


pub static mut ALIGNED_HEAP_START: *mut u8 = core::ptr::null_mut();

pub static mut PACKED_HEAP_START: *mut u8 = core::ptr::null_mut();

//around 200kb of DRAM0 on esp32-wroom32E (my device lil bro)
pub static mut USABLE_DRAM_END: *const u32 = core::ptr::null_mut();
pub static mut ALIGNED_HEAP_END: *mut u8 = core::ptr::null_mut();
pub static mut PACKED_HEAP_END: *mut u8 = core::ptr::null_mut();

pub static mut SIZE_OF_HEAP_BLOCK: usize = 32; //bits

fn usable_ram_end() -> *const u32 {
    //gets the stack pointer which indicates the end of the usable Sram main section
    return esp_hal::xtensa_lx::get_stack_pointer()
}
fn aligned_heap_start() -> *mut u8 {
    //gets the address to the first byte of sram/heap memory that I can use for allocs
    let start = unsafe { &_bss_end as *const u8 as usize } as *mut u8;

    //SAFETY this is reserved space for the HEAP_BLOCKS array
    return unsafe {start.byte_add(HEAP_BLOCK_AMOUNT)};
}

fn packed_heap_start() -> *mut u8 {
    let start = aligned_heap_start();
    unsafe {start.byte_add(ALIGNED_HEAP_SIZE + SMALL_BUFFER)}
}

fn packed_heap_end() -> *mut u8 {
    let end = unsafe {USABLE_DRAM_END.byte_sub(STACK_RESERVED)};
    return end as *mut u8;
}

pub fn packed_heap_usable_size() -> usize {
    let end = unsafe {USABLE_DRAM_END.byte_sub(packed_heap_start() as usize + STACK_RESERVED)};
    return end as usize;
}

fn aligned_heap_end() -> *mut u8{
    unsafe {
        let end = aligned_heap_start();
        return end.byte_add(ALIGNED_HEAP_SIZE);
    }
}





//CALL ONLY ONCE! ACTUALLY DON'T CALL IT AT ALL, EVER, LET ME DO IT
pub unsafe fn init_heap() {
    ALIGNED_HEAP_START = aligned_heap_start();
    PACKED_HEAP_START = packed_heap_start();
    
    USABLE_DRAM_END = usable_ram_end();
    
    ALIGNED_HEAP_END = aligned_heap_end();
    PACKED_HEAP_END = packed_heap_end();
}




#[derive(Debug)]
pub enum AllocError{
    SegmentationError,
    OutOfMemory,
    AlignmentError,
}

pub unsafe trait Allocator{
    fn allocate(&mut self, size_in_bytes: usize) -> Result<NonNull<[u8]>, AllocError>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, size_in_bytes: usize);

    //fn allocate_zeroed(&self, layout: layout::Layout) -> Result<NonNull<[u8]>, AllocError> {
    //    let ptr = self.allocate(layout)?;
    //}
}
#[derive(Debug)]
pub struct StandardAllocator{
    last_full_block_index: usize,
    last_sub_block_index: usize,
}

pub static mut STALLOC: StandardAllocator = StandardAllocator{last_full_block_index: 0, last_sub_block_index: 0};



unsafe impl Allocator for StandardAllocator {
    fn allocate(&mut self, size_in_bytes: usize) -> Result<NonNull<[u8]>, AllocError> {
        let needed_size = size_in_bytes;
        assert!(needed_size > 0, "you need to allocate more than 0 bytes");
        
        if needed_size >= ALIGNED_HEAP_SIZE{
            return Err(AllocError::OutOfMemory);
        }
        
        let full_blocks_to_alloc = needed_size / unsafe {SIZE_OF_HEAP_BLOCK};

        let rem = needed_size % unsafe {SIZE_OF_HEAP_BLOCK};
        let sub_blocks_to_alloc = if rem > 0 {
            (rem + 3) / 4 
        } else {
            0
        };
        
        let last_full_block_index = self.last_full_block_index;
        let last_sub_block_index = self.last_sub_block_index;
        unsafe {
            for i in 0..full_blocks_to_alloc{
                HEAP_BLOCKS[last_full_block_index + i] = u8::MAX;
            }
            if sub_blocks_to_alloc > 0{
                for _ in 0..sub_blocks_to_alloc{
                    HEAP_BLOCKS[full_blocks_to_alloc + last_full_block_index] = 2 << sub_blocks_to_alloc;
                }
            }
            self.last_full_block_index = last_full_block_index + full_blocks_to_alloc + if sub_blocks_to_alloc > 0 {1} else {0};
            self.last_sub_block_index = sub_blocks_to_alloc;
            
        
            info!("requested: {:?}", needed_size);
            let alloc_mem = full_blocks_to_alloc * unsafe {SIZE_OF_HEAP_BLOCK} + (sub_blocks_to_alloc * 4);
            info!("alloc_mem: {:?}", alloc_mem);
            let slice_start = ALIGNED_HEAP_START.byte_offset(alloc_mem as isize); 
            let slice = core::slice::from_raw_parts_mut(slice_start, alloc_mem);
            return Ok(NonNull::new_unchecked(slice))
         
        }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: usize) {

    }
}

    //struct vec<T, A: Allocator = StandardAllocator> {}
