# AnyList
 AnyList works as a Vec<T> but without generics in it's type.
 
 the only issue this creates is having to use the correct type
 in every function because all of them need a generic.
 
 capacity increments using a fibonacci sequence.
 
 # Example:
 ```
 fn main() {
     use anylist::AnyList;
     let mut list = AnyList::new::<u32>();

     list.push::<u32>(1);
     list.push::<u32>(2);

     assert_eq!(list.index::<u32>(0), 1)
     assert_eq!(list.index::<u32>(1), 2)
 }
 ```
