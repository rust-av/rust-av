//! Reference-counted buffer data type.
//!
//! Requires some reference-counted type especially for frame buffer pools.
//! `Arc` does not allow mutability with several references present,
//! `RwLock` does not work reliably in a single thread mode, so this file has
//! been created.
//!
//! Currently it does not prevent code from reading the data
//! that is being written to.
//! Maybe in the future this will be replaced by something better and
//! using more standard components.
//!
//! Also it contains `unsafe{}` code.
//!
//! # Examples
//!
//! ```
//! use av_data::buffer_ref::BufferRef;
//!
//! let vec = vec![42u8; 16];
//! let vec_ref = BufferRef::new(vec);
//! let vec_ref2 = vec_ref.clone();
//! // should be 2
//! let ref_count = vec_ref.get_num_refs();
//! // should print the fourth vector element
//! println!("vector element 4 is {}", vec_ref[4]);
//! ```

use std::convert::AsRef;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::*;

struct BufferData<T> {
    data: T,
    refs: AtomicUsize,
}

impl<T> BufferData<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            refs: AtomicUsize::new(1),
        }
    }
    fn inc_refs(obj: &mut Self) {
        obj.refs.fetch_add(1, Ordering::SeqCst);
    }
    fn dec_refs(obj: &mut Self) -> bool {
        obj.refs.fetch_sub(1, Ordering::SeqCst) == 1
    }
    fn get_num_refs(obj: &Self) -> usize {
        obj.refs.load(Ordering::Relaxed)
    }
    fn get_read_ptr(obj: &Self) -> &T {
        &obj.data
    }
    fn get_write_ptr(obj: &mut Self) -> Option<&mut T> {
        Some(&mut obj.data)
    }
}

/// Reference-counted buffer reference.
pub struct BufferRef<T> {
    ptr: *mut BufferData<T>,
}

unsafe impl<T> Sync for BufferRef<T> {}
unsafe impl<T> Send for BufferRef<T> {}

impl<T> BufferRef<T> {
    /// Constructs a new instance of `BufferRef`.
    pub fn new(val: T) -> Self {
        let bdata = BufferData::new(val);
        let nbox: Box<_> = Box::new(bdata);
        Self {
            ptr: Box::into_raw(nbox),
        }
    }
    /// Reports the number of references for the current instance.
    pub fn get_num_refs(&self) -> usize {
        unsafe { BufferData::get_num_refs(self.ptr.as_mut().unwrap()) }
    }
    /// Returns a mutable pointer to the underlying data if possible.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        unsafe { BufferData::get_write_ptr(self.ptr.as_mut().unwrap()) }
    }
}

impl<T> AsRef<T> for BufferRef<T> {
    fn as_ref(&self) -> &T {
        unsafe { BufferData::get_read_ptr(self.ptr.as_mut().unwrap()) }
    }
}

impl<T> Deref for BufferRef<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T> DerefMut for BufferRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut().unwrap()
    }
}

impl<T> Clone for BufferRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            BufferData::inc_refs(self.ptr.as_mut().unwrap());
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Drop for BufferRef<T> {
    fn drop(&mut self) {
        unsafe {
            if BufferData::dec_refs(self.ptr.as_mut().unwrap()) {
                let data = Box::from_raw(self.ptr);
                std::mem::drop(data);
            }
        }
    }
}

impl<T: Default> Default for BufferRef<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
