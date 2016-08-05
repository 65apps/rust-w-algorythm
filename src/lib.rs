extern crate libc;
extern crate regex;
extern crate crypto;

use std::ffi::CStr;
use std::fmt;
use regex::Regex;
use std::thread;
use crypto::md5::Md5;
use crypto::digest::Digest;

pub struct CompareText<'a> {
	text_a: &'a str,
	text_b: &'a str,
	shingles_length: usize,
	buffer_a: Buffer,
	buffer_b: Buffer,
}

pub enum Buffer {	
	Some(String),	
	None,
}

pub trait Canonize {
	fn delete_html(&mut self); 

	fn delete_stop_words(&mut self);
}

pub trait Matching {
	fn get_shingles(&self) -> Vec<Vec<String>>;

	fn get_similarities(&self, hashes: &Vec<Vec<String>>) -> f32;
}

impl<'a> CompareText<'a> {
	fn new(text_a: &'a str, text_b: &'a str, shingles_length: usize ) -> CompareText<'a> {
		CompareText {
			text_a: text_a,	
			text_b: text_b,
			shingles_length: shingles_length,
			buffer_a: Buffer::None,
			buffer_b: Buffer::None,
		}
	}
}

impl<'a> Canonize for CompareText<'a> {
	fn delete_html(&mut self) {
		let html = Regex::new(r"<[^>]*>|[:punct:]").unwrap();
		let mut without_html = html.replace_all(self.text_a, " ");

		self.buffer_a = Buffer::Some(without_html);		

		without_html = html.replace_all(self.text_b, " ");

		self.buffer_b = Buffer::Some(without_html);		
	} 	

	fn delete_stop_words(&mut self) {
		let stop_words = Regex::new(r"(?i)\b[а-я]{1,2}\b").unwrap();

		let mut new_value_a = String::new();
		if let &Buffer::Some(ref value) = &self.buffer_a {
			new_value_a = stop_words.replace_all(value, " ");			
		}

		self.buffer_a = Buffer::Some(new_value_a);
		

		let mut new_value_b = String::new();
		if let &Buffer::Some(ref value) = &self.buffer_b {
			new_value_b = stop_words.replace_all(value, " ");			
		}

		self.buffer_b = Buffer::Some(new_value_b);		
	}
}

impl<'a> Matching for CompareText<'a> {
	fn get_shingles(&self) -> Vec<Vec<String>> {
		let mut pack = vec![];

		if let &Buffer::Some(ref value) = &self.buffer_a {
			pack.push((value.clone(), self.shingles_length));
		} 		

		if let &Buffer::Some(ref value) = &self.buffer_b {
			pack.push((value.clone(), self.shingles_length));
		}

		let mut handles: Vec<_> = Vec::with_capacity(pack.len());
		for i in &pack {
			let data = i.clone();

			handles.push(
				thread::spawn(move || {  
					let split: Vec<&str> = data.0.split_whitespace().collect();

					let mut shingles = Vec::<String>::new();

					for i in 0..(split.len() - data.1 + 1) {
						let mut buf = String::new();
						for y in i..i + data.1 {
							buf = buf + " " + split[y];
						}

						let pair = String::from(buf.trim()).to_lowercase();

						let bytes: &[u8] = pair.as_bytes();
						let mut hash = Md5::new();
						hash.input(bytes);					
						
						shingles.push(hash.result_str());
					}

					shingles					
		        })			   	
			);			
		}

		let mut hashes: Vec<Vec<String>> = Vec::new();
		for h in handles {
			match h.join() {
				Ok(r) => hashes.push(r),
				Err(err) => println!("error {:?}", err),
			};
		}

		hashes		
	}

	fn get_similarities(&self, hashes: &Vec<Vec<String>>) -> f32 {
		for v in hashes {
			if (v.len() == 0 || hashes.len() != 2) {
				return 0 as f32
			}			
		}

		let shingles1 = &hashes[0];
		let shingles2 = &hashes[1];

		let mut same = 0;
		for item1 in shingles1 {
			for item2 in shingles2 {
				if(*item1 == *item2) {
					same += 1;
				}
			}
		}

		same = same*100;
		let length_text = std::cmp::min(shingles1.len(), shingles2.len()) as f32;		
		same as f32/length_text
	}
}

impl<'a> fmt::Display for CompareText<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {		

    	let display_a = match &self.buffer_a {
    	    &Buffer::Some(ref value) => value,
    	    &Buffer::None => "Buffer()",
    	};

    	let display_b = match &self.buffer_b {
    	    &Buffer::Some(ref value) => value,
    	    &Buffer::None => "Buffer()",
    	};

        write!(f, "CompareText (\n text_a: \"{}\",\n text_b: \"{}\",\n shingles_length: \"{}\",\n buffer_a: \"{}\",\n buffer_b: \"{}\"\n)", self.text_a, self.text_b, self.shingles_length, display_a, display_b )
    }
}

#[no_mangle]
pub extern "C" fn count_same(text1: *const libc::c_char, text2: *const libc::c_char, length: usize) -> f32 {
	let buf1 = unsafe { CStr::from_ptr(text1).to_bytes() };
	let text1_ = String::from_utf8(buf1.to_vec()).unwrap();

	let buf2 = unsafe { CStr::from_ptr(text2).to_bytes() };
	let text2_ = String::from_utf8(buf2.to_vec()).unwrap();

	let mut compare = CompareText::new(&text1_, &text2_, length);

	compare.delete_html();
	compare.delete_stop_words();

	let hashes = compare.get_shingles();

	compare.get_similarities(&hashes)	
}