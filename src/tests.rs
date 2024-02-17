use std::time::Instant;

use crate::AnyList;

fn insert_stress_box() {
    let start = Instant::now();
    let mut list: Vec<Box<usize>> = vec![];
    for i in 0..10000 {
        list.insert(0, Box::new(i));
    }
    let end: Instant = Instant::now();

    println!("insert box implementation: {:?}",(end - start).as_millis());
}

fn insert_stress_anylist() {
    let start = Instant::now();
    let mut list = AnyList::new::<usize>();
    for i in 0..10000 {
        list.insert::<usize>(0, i);
    }
    let end: Instant = Instant::now();

    println!("insert any implementation: {:?}",(end - start).as_millis());
}

fn insert_stress_vec() {
    let start = Instant::now();
    let mut list: Vec<usize> = vec![];
    for i in 0..10000 {
        list.insert(0, i);
    }
    let end: Instant = Instant::now();

    println!("insert vec implementation: {:?}",(end - start).as_millis());
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
        list.push::<usize>(0);
    }
    for _ in 0..10000 {
        list.untyped_remove(0);
    }
    let end: Instant = Instant::now();

    println!("remove any implementation: {:?}",(end - start).as_millis());
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

    println!("remove box implementation: {:?}",(end - start).as_millis());
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

    println!("remove vec implementation: {:?}",(end - start).as_millis());
}

fn remove_stress_comparison() {
    remove_stress_box();
    remove_stress_anylist();
    remove_stress_vec();
}

fn push_stress_anylist() {
    let start = Instant::now();
    let mut list = AnyList::new::<usize>();
    for i in 0..1000000 {
        list.push::<usize>(i);
    }
    let end: Instant = Instant::now();

    println!("push any implementation: {:?}",(end - start).as_millis());
}

fn push_stress_box() {
    let start = Instant::now();
    let mut list: Vec<Box<usize>> = vec![];
    for i in 0..1000000 {
        list.push(Box::new(i))
    }
    let end: Instant = Instant::now();

    println!("push box implementation: {:?}",(end - start).as_millis());
}

fn push_stress_vec() {
    let start = Instant::now();
    let mut list: Vec<usize> = vec![];
    for i in 0..1000000 {
        list.push(i)
    }
    let end: Instant = Instant::now();

    println!("push vec implementation: {:?}",(end - start).as_millis());
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
        list.untyped_pop();
    }
    let end: Instant = Instant::now();

    println!("pop any implementation: {:?}",(end - start).as_millis());
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

    println!("pop box implementation: {:?}",(end - start).as_millis());
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

    println!("pop vec implementation: {:?}",(end - start).as_millis());
}

fn pop_stress_comparison() {
    pop_stress_anylist();
    pop_stress_box();
    pop_stress_vec();
}

fn index_stress_anylist() {
    let start = Instant::now();
    let mut list: AnyList = AnyList::new::<usize>();
    for i in 0..1000000 {
        list.push::<usize>(i)
    }
    for i in 0..1000000 {
        assert_eq!(*list.get::<usize>(i).unwrap(), i) 
    }
    let end: Instant = Instant::now();

    println!("index any implementation: {:?}",(end - start).as_millis());
}

fn index_stress_vec() {
    let start = Instant::now();
    let mut list:Vec<usize> = vec![];
    for i in 0..1000000 {
        list.push(i)
    }
    for i in 0..1000000 {
        assert_eq!(list[i], i) 
    }
    let end: Instant = Instant::now();

    println!("index vec implementation: {:?}",(end - start).as_millis());
}

fn index_stress_box() {
    let start = Instant::now();
    let mut list:Vec<Box<usize>> = vec![];
    for i in 0..1000000 {
        list.push(Box::new(i))
    }
    for i in 0..1000000 {
        assert_eq!(*list[i], i) 
    }
    let end: Instant = Instant::now();

    println!("index box implementation: {:?}",(end - start).as_millis());
}

fn index_stress_comparison() {
    index_stress_box();
    index_stress_anylist();
    index_stress_vec();
}

#[test]
fn stress_comparisons() {
    push_stress_comparison();
    println!();
    pop_stress_comparison();
    println!();
    insert_stress_comparison();
    println!();
    remove_stress_comparison();
    println!();
    index_stress_comparison();
}

#[test]
fn general() {
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