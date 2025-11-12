#![no_std]
#![allow(static_mut_refs)]

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::NonNull;

#[repr(u8)]
pub enum BorrowState{
    Exclusive,
    Unshared,
    Shared(usize),
}

pub struct MemHandle<'handle, 'child, 'parent, V: 'child, P> 
    where 'parent: 'child,
          'child: 'handle
{
    mem: (*mut u8, usize), //pointer to first allocation byte, and capacity 
    state: BorrowState,
    parent: NonNull<P>,
    _phantom_child: PhantomData<&'child mut V>,
    _phantom_parent: PhantomData<&'parent UnsafeCell<P>>, 
    _phantom_handle: PhantomData<&'handle ()>,
}

impl<'handle, 'child, 'parent, V, P> Deref for MemHandle<'handle, 'child, 'parent, V, P>{
    type Target = P;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<'handle, 'child, 'parent, V, P> Drop for MemHandle<'handle, 'child, 'parent, V, P>{
    fn drop(&mut self){
        todo!()
    }
}

impl<'handle, 'child, 'parent, V, P: crate::alloc::Allocator> MemHandle<'handle, 'child, 'parent, V, P>{
    fn new(parent: NonNull<P>, mem: (*mut u8, usize), state: BorrowState) -> Self {
        return Self {
            state: BorrowState::Unshared,
            mem,
            parent,
            _phantom_child: PhantomData,
            _phantom_parent: PhantomData,
            _phantom_handle: PhantomData,
            
        }
    }
    unsafe fn drop_explicit(&mut self){
        
    }
}