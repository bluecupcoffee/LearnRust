use std::slice;
static HELLO_WORLD: &str = "Hello, world";
static mut COUNTER: u32 = 0;
fn main() {
    // unsafe uses

    // dereference a raw pointer
    // call an unsafe function or method
    // access or modify a mutable static var
    // implement an unsafe trait
    // access fields of a union

    // best to enclose unsafe code within a safe abstraction/API

    // dereferencing a raw pointer
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // calling an unsafe function or method
    unsafe fn dangerous () {}

    unsafe {
        dangerous();
    }

    // creating safe abstraction over unsafe code

    let mut v = vec![1,2,3,4,5];

    let r = &mut v[..];

    let (a,b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5]);
    println!("Hello, world!");

    let mut v2 = vec![1,2,3,4,5];
    let r2 = &mut v2[..];

    let (a2, b2) = split_at_mut2(r2, 3);

    // using extern functions to call external code
    // extern blocks are always unsafe to call from Rust code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // accessing or modifying a mutable static variable
    add_to_count(50);
    unsafe {
        println!("counter: {COUNTER}");
    }

}
// implementing an unsafe trait
unsafe trait Foo {}

unsafe impl Foo for i32 {}



fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}




extern "C" {
    fn abs(input: i32) -> i32;
}


fn split_at_mut2(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}
