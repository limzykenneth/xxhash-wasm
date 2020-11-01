use super::utils;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use twox_hash::XxHash32;
use std::hash::Hasher;

#[wasm_bindgen]
pub struct Hash32 {
	hasher: XxHash32
}

#[wasm_bindgen]
impl Hash32 {
	pub fn new(seed: u32) -> Hash32 {
		utils::set_panic_hook();

		Hash32 {
			hasher: XxHash32::with_seed(seed)
		}
	}

	pub fn hash_string(&self, data: String, seed: JsValue) -> f64 {
		let mut hasher;

		if seed.is_undefined() {
			hasher = XxHash32::with_seed(0);
		} else {
			let seed_val = match seed.as_f64() {
				Some(x) => x as u32,
				None => panic!("Seed must be a number"),
			};
			hasher = XxHash32::with_seed(seed_val);
		}

		hasher.write(data.as_bytes());
		let result = hasher.finish();
		result as f64
	}

	pub fn hash_bytes(&self, data: &[u8], seed: JsValue) -> f64 {
		let mut hasher;

		if seed.is_undefined() {
			hasher = XxHash32::with_seed(0);
		} else {
			let seed_val = match seed.as_f64() {
				Some(x) => x as u32,
				None => panic!("Seed must be a number"),
			};
			hasher = XxHash32::with_seed(seed_val);
		}

		hasher.write(data);
		let result =  hasher.finish();
		result as f64
	}

	pub fn init(&mut self, seed: u32) {
		self.hasher = XxHash32::with_seed(seed);
	}

	pub fn update_string(&mut self, data: String) {
		self.hasher.write(data.as_bytes());
	}

	pub fn update_bytes(&mut self, data: &[u8]) {
		self.hasher.write(data);
	}

	pub fn digest(&self) -> f64{
		self.hasher.finish() as f64
	}
}