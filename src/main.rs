#![feature(allocator_api)]
use bumpalo::Bump;
use std::alloc::{Allocator, Global, System};

mod my_arena;

fn main() {
    let bump = Bump::new();

    //  a bunch of boxes, each with their own allocator
    let b_sys: Box<u64, System> = Box::new_in(10_u64, System);
    let b_glb: Box<u64, Global> = Box::new_in(10_u64, Global);
    let b_bmp: Box<u64, &Bump> = Box::new_in(10_u64, &bump);

    println!("b_sys: {:?}", b_sys);
    println!("b_glb: {:?}", b_glb);
    println!("b_bmp: {:?}", b_bmp);

    // Some vecs
    let mut v_sys: Vec<u8, System> = Vec::new_in(System);

    v_sys.push(1);
    v_sys.push(2);

    println!("v_sys: {:?}", v_sys);

    let mut v_glb: Vec<u8, Global> = Vec::new_in(Global);

    v_glb.push(4);
    v_glb.push(8);

    println!("v_glb: {:?}", v_glb);

    let mut v_bmp: Vec<u8, &Bump> = Vec::new_in(&bump);

    v_bmp.push(16);
    v_bmp.push(32);

    println!("v_bpm: {:?}", v_bmp);

    let squares_sys: Vec<u32, System> = squares(System, 5);
    let squares_glb: Vec<u32, Global> = squares(Global, 5);
    let squares_bmp: Vec<u32, &Bump> = squares(&bump, 5);

    println!("squares_sys: {:?}", squares_sys);
    println!("squares_glb: {:?}", squares_glb);
    println!("squares_bmp: {:?}", squares_bmp);

    // Nested allocations aren't captured

    let thing: Vec<String, &Bump> = Vec::new_in(&bump); // String would be allocated in the GLobal still

    // Allocator is not dyn compatible - what would that even mean?
    //
    //
}

fn squares<A: Allocator>(alloc: A, n: u32) -> Vec<u32, A> {
    let mut v = Vec::new_in(alloc);
    v.extend((1..=n).map(|x| x * x));
    v
}
