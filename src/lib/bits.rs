#![no_std]
#![no_main]

use core::hint::unreachable_unchecked;
use core::mem;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitXorAssign, Not, Shl, Shr};

pub trait BITS{}

impl BITS for u8{}
impl BITS for u16{}
impl BITS for u32{}
impl BITS for u64{}
impl BITS for usize{}

impl BITS for i8{}
impl BITS for i16{}
impl BITS for i32{}
impl BITS for i64{}
impl BITS for isize{}

#[repr(u8)]

pub enum BIT{
    Zero = 0u8,
    One = 1u8,
}

/*
pub fn to_bits<'a, T>(value: T)-> &'a [u8] where T: BITS {
    
    let size = mem::size_of::<T>();
    let mut bit_array = [0; 64];
    let mut counter = 0;
    
    for byte in 0..size {
        for bit in 0..8 {
            let mask = 1 << (7 - bit);
            let bin = if (byte & mask) != 0 { 1 } else { 0 };
            bit_array[counter] = bin;
            counter += 1;
        }
    }
    return &bit_array;
}
*/

#[inline(always)]
pub fn tag_int<T>(mut val: T, offset: usize, bit: BIT) -> T
    where T: BITS
    + BitAnd<Output = T>
    + BitOr<Output = T>
    + Not<Output = T>
    + Shl<usize, Output = T>
    + From<u8>
{
    let bits = mem::size_of::<T>() * 8;
    assert!(offset < bits, "offset bigger than bits available!");

    let mask: T = T::from(1u8) << offset;

    match bit {
        BIT::One => val | mask,
        BIT::Zero => val & !mask,
    }
}

pub fn read_bit<T>(val: T, offset: usize) -> u8
        where T: BITS
        + BitAnd<Output = T>
        + Shr<usize, Output = T>
        + From<u8>
        + Into<u8>
        {
    let bits = mem::size_of::<T>() * 8;
    assert!(offset < bits, "offset bigger than bits available!");
    ((val >> offset) & T::from(1)).into()
}


pub fn write_int<T, V>(target: T, offset: usize, other: V)
    where T: BITS + Shl<u8, Output = V> + Shr<u8, Output = V> + PartialOrd,
          V: BITS + Shl<u8, Output = V> + Shr<u8, Output = V> + PartialOrd<T>
{
    assert!((other << 1u8) <= target);
    todo!()
}