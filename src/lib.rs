#![feature(get_many_mut)]

use std::{any::Any, ptr::copy, slice::{from_raw_parts, from_raw_parts_mut}};

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
    pub fn set<T: Any>(&mut self, index: usize, value: T) {
        unsafe { mem::transmute::<*mut u8, *mut T>(self.data).add(index).write(value) }
    }
    pub fn get<T: Any>(&mut self, index: usize) -> T {
        unsafe { mem::transmute::<*mut u8, *mut T>(self.data).add(index).read() }
    }
    fn no_mut_index_mut_unchecked<T: Any>(&self, index: usize) -> &mut T {
        unsafe { mem::transmute::<*mut u8, *mut T>(self.data).add(index).as_mut().unwrap() }
    }
    pub fn push<T: Any>(&mut self, item: T) {
        if self.len + 1 > self.capacity {
            let past_capacity = self.capacity;
            let new_capacity = self.capacity + self.past_capacity;
            self.reserve::<T>(new_capacity);
            self.past_capacity = past_capacity;
        }

        self.set::<T>(self.len, item);
        self.len += 1;
    }
    pub fn pop(&mut self) {
        self.len -= 1;
    }
    pub fn remove(&mut self, index: usize) {
        assert!(self.len > 0);
        
        unsafe {
            from_raw_parts_mut(self.data.add(index*self.item_size), self.item_size).fill(0)
        };
        
        for i in index..self.len {
            unsafe {
                from_raw_parts_mut(self.data.add(i*self.item_size), self.item_size).clone_from_slice(
                    from_raw_parts(self.data.add((i + 1)*self.item_size), self.item_size)
                );
            };
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
        for i in (index..self.len - 1).rev() {
            mem::swap(self.no_mut_index_mut_unchecked::<T>(i), self.no_mut_index_mut_unchecked(i+1))
        }
        
        *self.index_mut(index) = item;
    }
    pub const fn len(&self) -> usize {
        self.len
    }
    pub const fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::AnyList;

    #[test]
    fn stress() {
        let start = Instant::now();
        let mut list = AnyList::new::<usize>();
        for i in 0..10000 {
            list.push::<usize>(i);
        }
        for i in 0..10000 {
            assert!(*list.index::<usize>(i) == i); 
        }
        let end: Instant = Instant::now();

        println!("{:?}",(end - start).as_nanos());
    }

    #[test]
    fn stress_unoptimal() {
        let start = Instant::now();
        let mut list: Vec<Box<usize>> = vec![];
        for i in 0..10000 {
            list.push(Box::new(i));
        }
        for i in 0..10000 {
            assert!(*list[i] == i); 
        }
        let end: Instant = Instant::now();

        println!("{:?}",(end - start).as_nanos());
    }

    #[test]
    fn stress_ref() {
        let start = Instant::now();
        let mut list: Vec<usize> = vec![];
        for i in 0..10000 {
            list.push(i);
        }
        for i in 0..10000 {
            assert!(list[i] == i); 
        }
        let end: Instant = Instant::now();

        println!("{:?}",(end - start).as_nanos());
    }

    #[test]
    fn general() {
        let mut list = AnyList::new::<u32>();

        list.push::<u32>(1);
        list.push::<u32>(2);
        list.push::<u32>(3);

        assert_eq!(*list.index::<u32>(0), 1);
        assert_eq!(*list.index::<u32>(1), 2);
        assert_eq!(*list.index::<u32>(2), 3);

        list.remove(1);

        assert_eq!(*list.index::<u32>(0), 1);
        assert_eq!(*list.index::<u32>(1), 3);

        list.insert(1, 2);

        assert_eq!(*list.index::<u32>(0), 1);
        assert_eq!(*list.index::<u32>(1), 2);
        assert_eq!(*list.index::<u32>(2), 3);
    }
}