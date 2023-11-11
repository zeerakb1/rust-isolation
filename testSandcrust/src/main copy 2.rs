#[macro_use]
extern crate sandcrust;
extern crate libc;
// use libc::size_t;
use sandcrust::*;




extern {
    // Declaration of the external C function
    fn modify_vector(ptr: *mut i32, len: libc::size_t);
}

fn main() {
    let mut vec = vec![10, 11, 12, 13, 14, 15];
    println!("Original vector: {:?}", vec);

    unsafe {
        call_unsafe_function(&mut vec);
    }

	let potentially_invalid_value = unsafe { *vec.as_ptr().add(vec.len()) };
    println!("Garbage value read from out-of-bounds: {}", potentially_invalid_value);

    println!("Modified vector: {:?}", vec);

	// if vec.iter().all(|&x| x == 42) {
    //     println!("Vector modified by C function: {:?}", vec);
    // } else {
    //     println!("Vector not modified as expected: {:?}", vec);
    // }
}


unsafe fn call_unsafe_function(vec: &mut Vec<i32>) {
    modify_vector(vec.as_mut_ptr(), vec.len() as libc::size_t);
}


// use libc::{malloc, free};
// use std::mem;

// fn main() {
//     let size = 10;

//     unsafe {
        
// 		// Using `malloc` from the C library to allocate memory
//         let mem_ptr = malloc(mem::size_of::<u32>() * size) as *mut u32;

//         // Writing into allocated memory
// 		if !mem_ptr.is_null() {
//             for i in 0..size {
//                 *mem_ptr.add(i) = i as u32;
//             }

//             // Freeing the memory
//             free(mem_ptr as *mut libc::c_void);

//             // Attempting to access the memory after it has been freed
//             // This is undefined behavior
//             for i in 0..size {
//                 println!("{}", *mem_ptr.add(i)); // Dangling pointer access
//             }
//         }
//     }

//     // Rest of the Rust program...
// }



