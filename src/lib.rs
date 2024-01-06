use std::alloc::Layout;

/// AnyList works as a Vec<T> but without generics in it's type.
/// 
/// the only issue this creates is having to use the correct type
/// in every function because all of them(except for pop and drop)
/// need a generic.
/// 
/// this also used raw allocation, and the data references you can
/// get from functions can get duplicated thus breaking rust's memory
/// safety rules.
/// 
/// capacity increments using a fibonacci sequence.
/// 
/// # Example:
/// ```
/// fn main() {
///     use anylist::AnyList;
///     unsafe {
///         let mut list = AnyList::new::<u32>();
/// 
///         list.push::<u32>(1);
///         list.push::<u32>(2);
/// 
///         assert!(list.data::<u32>() == &[1,2])
///     }
/// }
/// ```

pub struct AnyList {
    data: *mut u8,
    capacity: usize,
    past_cap: usize,
    len: usize,
    layout: Layout
}

impl AnyList {
    /// creates a new list of capacity 1, and length 0.
    pub unsafe fn new<T>() -> AnyList {
        let layout = Layout::array::<T>(1).unwrap();
        let data: *mut u8 = std::alloc::alloc(layout);
        AnyList { data, capacity: 1, len: 0, past_cap: 1, layout }
    }
    pub unsafe fn data<T>(&self) -> &[T] {
        std::slice::from_raw_parts(self.data.cast::<T>(), self.len)
    }
    pub unsafe fn mut_data<T>(&self) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.data.cast::<T>(), self.len)
    }
    pub unsafe fn index<T>(&self, idx: usize) -> &T {
        assert!(idx < self.len);
        &self.data::<T>()[idx]
    }
    pub unsafe fn index_mut<T>(&mut self, idx: usize) -> &mut T {
        assert!(idx < self.len);
        &mut self.mut_data::<T>()[idx]
    }
    /// If tried to resize to a capacity minor or equal
    /// than the actual one, then nothing happens.
    pub unsafe fn resize<T>(&mut self, size: usize) {
        if self.capacity <= size {
            return;
        }
        let new_layout = Layout::array::<T>(size).unwrap();
        let new_data: *mut u8 = std::alloc::alloc_zeroed(new_layout);
        for i in 0..self.len * std::mem::size_of::<T>() {
            *new_data.add(i) = *self.data.add(i);
        }
        std::alloc::dealloc(self.data, Layout::array::<T>(self.capacity).unwrap());
        self.data = new_data;
        self.capacity = size;
        self.layout = new_layout;
    }
    pub unsafe fn push<T>(&mut self, item: T) {
        if self.len + 1 > self.capacity {
            let past = self.capacity;
            self.resize::<T>(self.capacity + self.past_cap);
            self.past_cap = past;
        }
        self.len += 1;
        *self.index_mut(self.len - 1) = item;
    }
    pub unsafe fn pop(&mut self) {
        assert!(self.len > 1);

        self.len -= 1;
    }
    pub unsafe fn remove<T>(&mut self, index: usize) {
        assert!(index < self.len);

        let data = self.mut_data::<T>();

        for i in index + 1..self.len {
            data.swap(i, i - 1);
        }

        self.len -= 1;
    }
    pub unsafe fn insert<T>(&mut self, index: usize, item: T) {
        assert!(index < self.len);

        if self.len + 1 > self.capacity {
            let past = self.capacity;
            self.resize::<T>(self.capacity + self.past_cap);
            self.past_cap = past;
        }

        self.len += 1;

        let data = self.mut_data::<T>();
        
        for i in (index..self.len - 1).rev() {
            data.swap(i, i + 1);
        }

        data[index] = item;
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Drop for AnyList {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.data, self.layout); }
    }
}

mod tests {
    #[test]
    fn push_test() {
        use crate::AnyList;

        unsafe {
            let mut list = AnyList::new::<u32>();

            list.push::<u32>(1);
            list.push::<u32>(2);

            assert!(list.data::<u32>() == &[1,2]);
        }
    }
    #[test]
    fn pop_test() {
        use crate::AnyList;

        unsafe {
            let mut list = AnyList::new::<u32>();

            list.push::<u32>(1);
            list.push::<u32>(2);
            list.push::<u32>(3);

            list.pop();

            assert!(list.data::<u32>() == &[1,2]);
        }
    }
    #[test]
    fn remove_test() {
        use crate::AnyList;

        unsafe {
            let mut list = AnyList::new::<u32>();

            list.push::<u32>(1);
            list.push::<u32>(2);
            list.push::<u32>(3);

            list.remove::<u32>(1);

            assert!(list.data::<u32>() == &[1,3]);
        }
    }
    #[test]
    fn insert_test() {
        use crate::AnyList;

        unsafe {
            let mut list = AnyList::new::<u32>();

            list.push::<u32>(1);
            list.push::<u32>(3);

            list.insert::<u32>(1, 2);

            assert!(list.data::<u32>() == &[1,2,3]);
        }
    }
}