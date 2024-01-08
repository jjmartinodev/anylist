# AnyList
 AnyList works as a Vec<T> but without generics in it's type.
 
 the only issue this creates is having to use the correct type
 in every function because all of them need a generic, exept for
 the remove and pop implementations.
 
 capacity increments using a fibonacci sequence.
 
 # Example:
 ```
 fn main() {
     use anylist::AnyList;
     let mut list = AnyList::new::<u32>();

     list.push::<u32>(1);
     list.push::<u32>(2);
     list.push::<u32>(3);

     assert_eq!(*list.index::<u32>(0), 1);
     assert_eq!(*list.index::<u32>(1), 3);
     assert_eq!(*list.index::<u32>(2), 0);

     list.remove(1);

     assert_eq!(*list.index::<u32>(0), 1);
     assert_eq!(*list.index::<u32>(1), 3);
     assert_eq!(*list.index::<u32>(2), 0);
 }
 ```
