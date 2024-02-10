use std::{any::Any, ptr::{self, copy}};

use std::alloc;
use std::mem;

pub struct AnyList {
    data:*mut u8,
    len: usize,
    capacity: usize,
    past_capacity: usize,
    item_size: usize,
}

impl AnyList {
    fn alloc<T: Any>(size: usize) -> *mut u8 {
        let layout = alloc::Layout::array::<T>(size).unwrap();
        unsafe { alloc::alloc_zeroed(layout) }
    }
    fn dealloc<T: Any>(size: usize, data: *mut u8) {
        let layout = alloc::Layout::array::<T>(size).unwrap();
        unsafe { alloc::dealloc(data, layout) }
    }
    pub fn new<T: Any>() -> AnyList {
        AnyList {
            data: Self::alloc::<T>(1),
            len: 0,
            capacity: 1,
            past_capacity: 1,
            item_size: mem::size_of::<T>()
        }
    }
    pub fn reserve<T: Any>(&mut self, capacity: usize) {
        if self.capacity > capacity {
            return
        }
        
        let new_data = Self::alloc::<T>(capacity);

        unsafe { copy(self.data, new_data, self.capacity * mem::size_of::<T>()); }
        Self::dealloc::<T>(self.capacity, self.data);
        
        self.capacity = capacity;
        self.data = new_data;
    }
    pub fn index<T: Any>(&self, index: usize) -> &T {
        assert!(index < self.len);
        unsafe { mem::transmute::<*mut u8, *mut T>(self.data).add(index).as_ref().unwrap() }
    }
    pub fn index_mut<T: Any>(&mut self, index: usize) -> &mut T {
        assert!(index < self.len);
        unsafe { mem::transmute::<*mut u8, *mut T>(self.data).add(index).as_mut().unwrap() }
    }
    pub fn index_unchecked<T: Any>(&self, index: usize) -> &T {
        unsafe { mem::transmute::<*mut u8, *mut T>(self.data).add(index).as_ref().unwrap() }
    }
    pub fn index_mut_unchecked<T: Any>(&mut self, index: usize) -> &mut T {
        unsafe { mem::transmute::<*mut u8, *mut T>(self.data).add(index).as_mut().unwrap() }
    }
    pub fn push<T: Any>(&mut self, item: T) {
        if self.len + 1 > self.capacity {
            let past_capacity = self.capacity;
            let new_capacity = self.capacity + self.past_capacity;
            self.reserve::<T>(new_capacity);
            self.past_capacity = past_capacity;
        }

        unsafe { self.data.cast::<T>().add(self.len).write(item) };
        self.len += 1;
    }
    pub fn pop(&mut self) {
        unsafe { self.data.add((self.len - 1) * self.item_size).write_bytes(0, self.item_size) }
        self.len -= 1;
    }
    pub fn remove(&mut self, index: usize) {
        assert!(self.len > 0);
        
        let bytes_moved = (self.len - index) * self.item_size;

        unsafe {
            ptr::copy(
                self.data.add((index + 1) * self.item_size),
                self.data.add(index * self.item_size),
                bytes_moved
            );
        }
        self.len -= 1;
    }
    pub fn insert<T: Any>(&mut self, index: usize, item: T) {
        if self.len + 1 > self.capacity {
            let past_capacity = self.capacity;
            let new_capacity = self.capacity + self.past_capacity;
            self.reserve::<T>(new_capacity);
            self.past_capacity = past_capacity;
        }

        self.len += 1;
        let casted = self.data.cast::<T>();
        let bytes_moved = self.len - index - 1;
        unsafe {
            ptr::copy(
                casted.add(index),
                casted.add(index + 1),
                bytes_moved
            );
            casted.add(index).write(item);
        }
        
    }
    pub const fn len(&self) -> usize {
        self.len
    }
    pub const fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests;