# AnyList
 AnyList works as a Vec<T> but without generics in it's type.
 
 without having a generic it's type, you can make a list of lists that
 have different types.

 the only issue this creates is having to use the correct type
 in every function because all of them need a generic, exept for
 the remove and pop implementations.

 the only purpose of this is to out perform an Vec<Box<Any>>, which
 generates fragmentation and indirection, with an extra pointer.
 
 capacity increments using a fibonacci sequence.
 
 # Example:
 ```
 fn main() {
   let mut list = AnyList::new::<u32>();

   list.push::<u32>(1);
   list.insert::<u32>(1, 2);
   list.push::<u32>(3);

   assert_eq!(*list.index::<u32>(0), 1);
   assert_eq!(*list.index::<u32>(1), 2);
   assert_eq!(*list.index::<u32>(2), 3);

   list.remove(0);

   assert_eq!(*list.index::<u32>(0), 2);
   assert_eq!(*list.index::<u32>(1), 3);

   list.pop();

   assert_eq!(*list.index::<u32>(0), 2);
   
   list.insert::<u32>(0, 1);
   
   for i in 0..list.len() {
      println!("{:?}", list.index::<u32>(i))
   }

   assert_eq!(*list.index::<u32>(0), 1);
   assert_eq!(*list.index::<u32>(1), 2);
 }
 ```
