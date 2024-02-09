# AnyList
 AnyList works as a Vec<T> but without generics in it's type.
 
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
 ```
