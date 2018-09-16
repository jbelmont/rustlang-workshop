use std::slice;

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    let numbers: &mut[i32; 5] = &mut[1, 2, 3, 4, 5];

    let numbers = split_at_mut(numbers, 2);

    println!("{:?}", numbers);
}
