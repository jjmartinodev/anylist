# AnyList
 AnyList works as a Vec<T> but without generics in it's type.
 
 without having a generic it's type, you can make a list of lists that
 have different types.

 the only issue this creates is having to use the correct type
 in every function because all of them need a generic, exept for
 the implementations that don't need it on their arguments(ej: remove, pop).

 if a wrong type it's used, then the API will panic.

 the only purpose of this is to out perform an Vec<Box<Any>>, which
 generates fragmentation and indirection, with an extra pointer. and
 to be able to use implementations like pop and remove, without needing
 generics in their functions.

 implementation made by SkiFire13 in reddit.
 
 # Example:
 ```
 fn main() {
    let mut list = AnyList::new::<u32>();

    list.push::<u32>(1);
    list.insert::<u32>(1, 2);
    list.push::<u32>(3);

    assert_eq!(list.as_slice::<u32>(), &[1,2,3]);

    list.untyped_remove(0);

    assert_eq!(list.as_slice::<u32>(), &[2,3]);

    list.untyped_pop();

    assert_eq!(list.as_slice::<u32>(), &[2]);
    
    list.insert::<u32>(0, 1);
    
    for i in 0..list.len() {
      println!("{:?}", list.get::<u32>(i).unwrap())
    }

    assert_eq!(list.as_slice::<u32>(), &[1,2]);
 }
 ```
