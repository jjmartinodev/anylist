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
    unsafe fn ix(&mut self, index: usize) -> usize {
        self.item_size * index
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
        self.len -= 1;
    }
    pub fn remove(&mut self, index: usize) {
        assert!(self.len > 0);

        unsafe {
            let bytes_moved = self.len - index - 1;
            ptr::copy(
                self.data.add(self.ix(index + 1)),
                self.data.add(self.ix(index)),
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
        let bytes_moved = self.len - index;
        unsafe {
            ptr::copy(
                self.data.add(index),
                self.data.add(index + 1),
                bytes_moved
            );
            self.data.cast::<T>().add(index).write(item);
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
mod tests {
    use std::time::Instant;

    use crate::AnyList;

    fn insert_stress_box() {
        let start = Instant::now();
        let mut list: Vec<Box<usize>> = vec![];
        for i in 0..10000 {
            list.insert(0, Box::new(i));
        }
        let end: Instant = Instant::now();

        println!("insert box implementation: {:?}",(end - start).as_nanos());
    }
    
    fn insert_stress_anylist() {
        let start = Instant::now();
        let mut list = AnyList::new::<usize>();
        for i in 0..10000 {
            list.insert::<usize>(0, i);
        }
        let end: Instant = Instant::now();

        println!("insert any implementation: {:?}",(end - start).as_nanos());
    }

    fn insert_stress_vec() {
        let start = Instant::now();
        let mut list: Vec<usize> = vec![];
        for i in 0..10000 {
            list.insert(0, i);
        }
        let end: Instant = Instant::now();

        println!("insert vec implementation: {:?}",(end - start).as_nanos());
    }

    fn insert_stress_comparison() {
        insert_stress_vec();
        insert_stress_anylist();
        insert_stress_box();
    }

    fn remove_stress_anylist() {
        let start = Instant::now();
        let mut list = AnyList::new::<usize>();
        for _ in 0..10000 {
            list.push(0);
        }
        for _ in 0..10000 {
            list.remove(0);
        }
        let end: Instant = Instant::now();

        println!("remove any implementation: {:?}",(end - start).as_nanos());
    }

    fn remove_stress_box() {
        let start = Instant::now();
        let mut list: Vec<Box<usize>> = vec![];
        for _ in 0..10000 {
            list.push(Box::new(0));
        }
        for _ in 0..10000 {
            list.remove(0);
        }
        let end: Instant = Instant::now();

        println!("remove box implementation: {:?}",(end - start).as_nanos());
    }

    fn remove_stress_vec() {
        let start = Instant::now();
        let mut list: Vec<usize> = vec![];
        for _ in 0..10000 {
            list.push(0);
        }
        for _ in 0..10000 {
            list.remove(0);
        }
        let end: Instant = Instant::now();

        println!("remove vec implementation: {:?}",(end - start).as_nanos());
    }

    fn remove_stress_comparison() {
        remove_stress_anylist();
        remove_stress_box();
        remove_stress_vec();
    }

    fn push_stress_anylist() {
        let start = Instant::now();
        let mut list = AnyList::new::<usize>();
        for i in 0..1000000 {
            list.push::<usize>(i);
        }
        let end: Instant = Instant::now();

        println!("push any implementation: {:?}",(end - start).as_nanos());
    }

    fn push_stress_box() {
        let start = Instant::now();
        let mut list: Vec<Box<usize>> = vec![];
        for i in 0..1000000 {
            list.push(Box::new(i))
        }
        let end: Instant = Instant::now();

        println!("push box implementation: {:?}",(end - start).as_nanos());
    }

    fn push_stress_vec() {
        let start = Instant::now();
        let mut list: Vec<usize> = vec![];
        for i in 0..1000000 {
            list.push(i)
        }
        let end: Instant = Instant::now();

        println!("push vec implementation: {:?}",(end - start).as_nanos());
    }

    fn push_stress_comparison() {
        push_stress_anylist();
        push_stress_box();
        push_stress_vec();
    }

    fn pop_stress_anylist() {
        let start = Instant::now();
        let mut list = AnyList::new::<usize>();
        for i in 0..1000000 {
            list.push::<usize>(i);
        }
        for _ in 0..1000000 {
            list.pop();
        }
        let end: Instant = Instant::now();

        println!("push any implementation: {:?}",(end - start).as_nanos());
    }

    fn pop_stress_box() {
        let start = Instant::now();
        let mut list: Vec<Box<usize>> = vec![];
        for i in 0..1000000 {
            list.push(Box::new(i))
        }
        for _ in 0..1000000 {
            list.pop();
        }
        let end: Instant = Instant::now();

        println!("push box implementation: {:?}",(end - start).as_nanos());
    }

    fn pop_stress_vec() {
        let start = Instant::now();
        let mut list: Vec<usize> = vec![];
        for i in 0..1000000 {
            list.push(i)
        }
        for _ in 0..1000000 {
            list.pop();
        }
        let end: Instant = Instant::now();

        println!("push vec implementation: {:?}",(end - start).as_nanos());
    }

    fn pop_stress_comparison() {
        pop_stress_anylist();
        pop_stress_box();
        pop_stress_vec();
    }

    #[test]
    fn stress_comparisons() {
        push_stress_comparison();
        pop_stress_comparison();
        insert_stress_comparison();
        remove_stress_comparison();
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