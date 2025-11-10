use core::mem;
use core::ops::BitAnd;

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


fn tag_int<T>(mut val: T, offset: usize, bit: BIT) -> T where T: BITS + BitAnd<u64, Output = T> {
    let bits = mem::size_of::<T>() * 8;
    assert!(offset <= bits, "offset bigger than bits available!");
    let b = match bit {
        BIT::One => 1,
        BIT::Zero => 0,
    };
    let mask = b << offset;
    val = val & mask;

    return val
}


#[cfg(test)]
mod test {
    use log::info;

    #[test]
    fn test_tag_int() {
        info!("hey");
        let result = tag_int(70, 5, BIT::One);
        assert_eq!(86, result);
    }
}