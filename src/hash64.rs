use super::utils;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use twox_hash::XxHash64;
use std::hash::Hasher;

#[wasm_bindgen]
pub struct Hash64 {
	hasher: XxHash64
}

#[wasm_bindgen]
impl Hash64 {
	pub fn new(seed: u64) -> Hash64 {
		utils::set_panic_hook();

		Hash64 {
			hasher: XxHash64::with_seed(seed)
		}
	}

	pub fn hash_string(&self, data: String, seed: JsValue) -> String {
		let mut hasher;

		if seed.is_undefined() {
			hasher = XxHash64::with_seed(0);
		} else {
			let seed_val = match seed.as_f64() {
				Some(x) => x as u64,
				None => panic!("Seed must be a number"),
			};
			hasher = XxHash64::with_seed(seed_val);
		}

		hasher.write(data.as_bytes());
		format!("{:x}", hasher.finish())
	}

	pub fn hash_bytes(&self, data: &[u8], seed: JsValue) -> String {
		let mut hasher;

		if seed.is_undefined() {
			hasher = XxHash64::with_seed(0);
		} else {
			let seed_val = match seed.as_f64() {
				Some(x) => x as u64,
				None => panic!("Seed must be a number"),
			};
			hasher = XxHash64::with_seed(seed_val);
		}

		hasher.write(data);
		format!("{:x}", hasher.finish())
	}

	pub fn init(&mut self, seed: u64) {
		self.hasher = XxHash64::with_seed(seed);
	}

	pub fn update_string(&mut self, data: String) {
		self.hasher.write(data.as_bytes());
	}

	pub fn update_bytes(&mut self, data: &[u8]) {
		self.hasher.write(data);
	}

	pub fn digest(&self) -> String {
		format!("{:x}", self.hasher.finish())
	}
}