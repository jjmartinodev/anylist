#![feature(get_many_mut)]

use std::{any::{TypeId, Any}, mem::{size_of, swap}};

pub struct AnyList {
    data:Box<[u8]>,
    len: usize,
    typeid: TypeId,
    capacity: usize,
    past_capacity: usize,
    item_size: usize,
}

impl AnyList {
    pub fn new<T: Any>() -> AnyList {
        let typeid = TypeId::of::<T>();
        AnyList {
            data: vec![0u8;size_of::<T>()].into_boxed_slice(),
            len: 0,
            typeid,
            capacity: 1,
            past_capacity: 1,
            item_size: std::mem::size_of::<T>()
        }
    }
    pub fn expand<T: Any>(&mut self, size: usize) {
        assert_eq!(self.typeid, TypeId::of::<T>());

        if self.capacity >= size {
            return;
        }

        let mut new_data = vec![0u8;size_of::<T>() * size].into_boxed_slice();

        for i in 0..self.len * size_of::<T>() {
            new_data[i] = self.data[i];
        }

        self.data = new_data;

        self.capacity = size;
    }
    pub fn index<T: Any>(&self, index: usize) -> &T {
        assert_eq!(self.typeid, TypeId::of::<T>());
        unsafe {
            &std::mem::transmute::<&Box<[u8]>, &Box<[T]>>(&self.data)[index]
        }
    }
    pub fn index_mut<T: Any>(&mut self, index: usize) -> &mut T {
        assert_eq!(self.typeid, TypeId::of::<T>());
        unsafe {
            &mut std::mem::transmute::<&mut Box<[u8]>, &mut Box<[T]>>(&mut self.data)[index]
        }
    }
    pub fn push<T: Any>(&mut self, item: T) {
        assert_eq!(self.typeid, TypeId::of::<T>());
        if self.len + 1 >= self.capacity {
            let past_capacity = self.capacity;
            self.expand::<T>(self.capacity + self.past_capacity);
            self.past_capacity = past_capacity;
        }

        self.len += 1;
        *self.index_mut::<T>(self.len - 1) = item;
    }
    pub fn pop(&mut self) {
        assert!(self.len > 0);
        self.len -= 1;
    }
    pub fn remove(&mut self, index: usize) {
        assert!(self.len > 0);

        let mut chunks: Vec<&mut [u8]> = self.data.chunks_mut(self.item_size).collect::<Vec<_>>();

        let mut temp: Vec<u8> = vec![0u8; self.item_size];
        for i in index..self.len {
            temp.clone_from_slice(chunks[i]);
            let [a, b] = chunks.get_many_mut([i,i+1]).unwrap();
            a.clone_from_slice(&b);
            b.clone_from_slice(&temp);
        }
    }
    pub fn insert<T: Any>(&mut self, index: usize, item: T) {
        assert_eq!(self.typeid, TypeId::of::<T>());
        if self.len + 1 >= self.capacity {
            let past_capacity = self.capacity;
            self.expand::<T>(self.capacity + self.past_capacity);
            self.past_capacity = past_capacity;
        }

        let data = unsafe {
            std::mem::transmute::<&mut Box<[u8]>, &mut Box<[T]>>(&mut self.data)
        };

        for i in (index + 1..=self.len).rev() {
            swap(&mut data[i],&mut self.index_mut::<T>(i-1));
        }

        data[index] = item;
        self.len += 1;
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
        let mut list = AnyList::new::<u32>();
        let start = Instant::now();
        for i in 0..10000 {
            list.insert::<u32>(0, i);
        }
        for _ in 0..10000 {
            list.pop();
        }
        let end = Instant::now();
        println!("{:?}",(end - start).as_millis());
    }

    #[test]
    fn general() {
        let mut list = AnyList::new::<u32>();
        list.push::<u32>(1);
        list.push::<u32>(2);
        list.push::<u32>(3);
        list.push::<u32>(4);
        list.remove(2);
        for i in 0..list.len() {
            println!("{}",list.index::<u32>(i));
        }
    }
}