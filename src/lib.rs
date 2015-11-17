extern crate libc;
extern crate regex;
extern crate crypto;

use std::ffi::CStr;

use std::thread;
use regex::Regex;
use std::cmp::min;
use crypto::md5::Md5;
use crypto::digest::Digest;

#[no_mangle]
pub extern "C" fn count_same(text1: *const libc::c_char, text2: *const libc::c_char, length: usize) -> f32 {
	let buf1 = unsafe { CStr::from_ptr(text1).to_bytes() };
	let text1_ = String::from_utf8(buf1.to_vec()).unwrap();

	let buf2 = unsafe { CStr::from_ptr(text2).to_bytes() };
	let text2_ = String::from_utf8(buf2.to_vec()).unwrap();

	fn canonize(text: String) -> String {
		let html = Regex::new(r"<[^>]*>|[:punct:]").unwrap();
		let stop_words = Regex::new(r"(?i)\b[а-я]{1,2}\b").unwrap();
		let mut temp = html.replace_all(&text, " ");
		temp = stop_words.replace_all(&temp, " ");
		temp
	}

	fn get_shingles(text: String, len: usize) -> Vec<String> {
		let text = canonize(text);
		let split: Vec<&str> = text.split_whitespace().collect();
		if(split.len()<len) {
			return Vec::new();
		}

		let mut str: Vec<String> = Vec::new();
		for i in 0..(split.len()-len+1) {
			let mut buf = String::new();
			for y in i..i+len {
				buf = buf + " " + split[y];
			}

			let el = String::from(buf.trim()).to_lowercase();
			str.push(el);
		}

		let mut handles: Vec<_> = Vec::with_capacity(str.len());
		for item in str {
			handles.push(
				thread::spawn(move || {
					let bytes: &[u8] = item.as_bytes();
					let mut hash = Md5::new();
					hash.input(bytes);
					hash.result_str()
				})
			)
		}

		let mut res: Vec<String> = Vec::new();
		for h in handles {
			match h.join() {
				Ok(r) => res.push(r),
				Err(err) => println!("error {:?}", err),
			};
		}
		res
	}

	let shingles1 = get_shingles(text1_, length);
	let shingles2 = get_shingles(text2_, length);
	if(shingles1.len()==0 || shingles2.len()==0) {
		return 0 as f32;
	}

	let mut same = 0;
	for item in &shingles1 {
		for el in &shingles2 {
			if(*item == *el) {
				same += 1;
			}
		}
	}

	same = same*100;
	let length_text = min(shingles1.len(), shingles2.len());
	let length_text_f = length_text as f32;
	let same_f = same as f32;

	let result: f32 = same_f/length_text_f;
	result
}