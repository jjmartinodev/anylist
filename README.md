# AnyList
 AnyList works as a Vec<T> but without generics in it's type.
 
 the only issue this creates is having to use the correct type
 in every function because all of them(except for pop and drop)
 need a generic.
 
 this also used raw allocation, and the data references you can
 get from functions can get duplicated thus breaking rust's memory
 safety rules.
 
 capacity increments using a fibonacci sequence.
 
 # Example:
 ```
 fn main() {
     use anylist::AnyList;
     unsafe {
         let mut list = AnyList::new::<u32>();
 
         list.push::<u32>(1);
         list.push::<u32>(2);
 
         assert!(list.data::<u32>() == &[1,2])
     }
 }
 ```
