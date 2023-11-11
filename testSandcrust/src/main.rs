#[macro_use]
extern crate libc;
extern crate sandcrust;

use sandcrust::*;
use libc::{c_int, size_t};


#[link(name = "snappy")]
extern {
	fn snappy_compress(input: *const u8,
		input_length: size_t,

		compressed: *mut u8,
		compressed_length: *mut size_t) -> c_int;
    fn snappy_uncompress(compressed: *const u8,
                         compressed_length: size_t,
                         uncompressed: *mut u8,
                         uncompressed_length: *mut size_t) -> c_int;
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
    fn snappy_uncompressed_length(compressed: *const u8,
                                  compressed_length: size_t,
                                  result: *mut size_t) -> c_int;
    fn snappy_validate_compressed_buffer(compressed: *const u8,
                                         compressed_length: size_t) -> c_int;
}

pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    unsafe {
        snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
    }
}


sandbox!{
	pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
		unsafe {
			let srclen = src.len() as size_t;
			let psrc = src.as_ptr();

			let mut dstlen: size_t = 0;
			snappy_uncompressed_length(psrc, srclen, &mut dstlen);

			let mut dst = Vec::with_capacity(dstlen as usize);
			let pdst = dst.as_mut_ptr();

			if snappy_uncompress(psrc, srclen, pdst, &mut dstlen) == 0 {
				dst.set_len(dstlen as usize);
				Some(dst)
			} else {
				None
			}
		}
	}
}

pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen = snappy_max_compressed_length(srclen);
        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        snappy_compress(psrc, srclen, pdst, &mut dstlen);
        
        // Setting the length incorrectly (Adding 10 bytes more than actual length)
        dst.set_len(dstlen as usize + 10);

        dst
    }
}

fn main() {
	let data = b"I dont know what I'm doing";
	sandcrust_init();
    let compressed = compress(data);
	sandcrust_terminate();
    println!("Compressed data: {:?}", compressed);
	

}


