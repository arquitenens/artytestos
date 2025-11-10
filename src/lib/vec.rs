#![no_std]
#![allow(static_mut_refs)]

use core::fmt::{Debug, Formatter};
use core::ptr::{null_mut, NonNull, Unique};
use crate::alloc;
use crate::alloc::{AllocError, Allocator, StandardAllocator, STALLOC};
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use esp_println::println;
use crate::alignment::{Alignment, AlignmentEnum};


#[derive(Debug)]
pub struct Vec<T, A: Allocator = StandardAllocator> {
    inner: VecInner<T, A>,
    len: usize,
}
#[derive(Debug)]
struct VecInner<T, A: Allocator = StandardAllocator> {
    ptr: NonNull<u8>,
    capacity: usize,
    _alloc: PhantomData<A>,
    _size: PhantomData<T>,
}


unsafe impl<T, A: Allocator> Allocator for VecInner<T, A> {
    fn allocate(&mut self, size_in_bytes: usize) -> Result<NonNull<[u8]>, AllocError> {
        let global = unsafe {&mut STALLOC};
        return global.allocate(size_in_bytes)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, size_in_bytes: usize) {
        todo!()
    }
}

const RESERVE_SPACE_FOR_N_T: usize = 2;

const DEFAULT_CAPACITY: fn(usize) -> usize = |T: usize|{
    return T * RESERVE_SPACE_FOR_N_T;
};

impl<T> Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        unsafe {
            let raw = self.inner.ptr.as_ptr() as *mut T;
            return &*raw.byte_add(i * size_of::<T>())
        }
    }
}

impl<T> IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        unsafe {
            let raw = self.inner.ptr.as_ptr() as *mut T;
            return &mut *raw.byte_add(i * size_of::<T>())
        }
    }
}



impl<T, A: Allocator> Vec<T, A> {
    pub fn new() -> Vec<T, A> {
        Self::with_capacity(DEFAULT_CAPACITY(size_of::<T>()))
    }
    ///Capacity is in bytes
    pub fn with_capacity(capacity: usize) -> Vec<T, A> {
        let mut inner = VecInner{
            ptr: NonNull::dangling(),
            capacity: 0,
            _alloc: PhantomData,
            _size: PhantomData,
        };
        let allocation = match inner.allocate(capacity * size_of::<T>()) {
            Ok(ptr) => ptr,
            Err(e) => panic!("{:?}", e),
        };
        let first = unsafe {allocation.as_ref().first()};
        inner.ptr = NonNull::from_ref(first.unwrap());
        inner.capacity = allocation.len() / size_of::<T>();
        return Vec{
            inner,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        unsafe {
            assert!(self.len < self.inner.capacity, "makeshift protection for now!");
            let raw = self.inner.ptr.as_ptr() as *mut T;

            let ptr = raw.byte_add(size_of::<T>() * self.len);
            ptr.write(value);
            self.len += 1;
            
        }
    }
}

/*

Broken, might fix in a bit!

impl<T, const N: usize> From<[T; N]> for Vec<T> {
    fn from(mut slice: [T; N]) -> Vec<T> {
        unsafe {
            let mut vec: Vec<T> = Vec::with_capacity(N);
            let raw = vec.inner.ptr.as_ptr() as *mut T;
            let ptr = raw.byte_add(size_of::<T>() * N);
            slice.as_mut_ptr().copy_to(raw, N * size_of::<T>());
            vec.inner.ptr = NonNull::new_unchecked(ptr as *mut u8);
            vec.len = N;
            return vec
        }
    }
}

 */

 
