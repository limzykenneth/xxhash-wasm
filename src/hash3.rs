use super::utils;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use twox_hash::Xxh3Hash64;
use std::hash::Hasher;

#[wasm_bindgen]
pub struct Hash3 {
	hasher: Xxh3Hash64
}

#[wasm_bindgen]
impl Hash3 {
	pub fn new(seed: u64) -> Hash3 {
		utils::set_panic_hook();

		Hash3 {
			hasher: Xxh3Hash64::with_seed(seed)
		}
	}

	pub fn hash_string(&self, data: String, seed: JsValue) -> u64 {
		let mut hasher;

		if seed.is_undefined() {
			hasher = Xxh3Hash64::with_seed(0);
		} else {
			let seed_val = match seed.as_f64() {
				Some(x) => x as u64,
				None => panic!("Seed must be a number"),
			};
			hasher = Xxh3Hash64::with_seed(seed_val);
		}

		hasher.write(data.as_bytes());
		hasher.finish()
	}

	pub fn hash_bytes(&self, data: &[u8], seed: JsValue) -> u64 {
		let mut hasher;

		if seed.is_undefined() {
			hasher = Xxh3Hash64::with_seed(0);
		} else {
			let seed_val = match seed.as_f64() {
				Some(x) => x as u64,
				None => panic!("Seed must be a number"),
			};
			hasher = Xxh3Hash64::with_seed(seed_val);
		}

		hasher.write(data);
		hasher.finish()
	}

	pub fn init(&mut self, seed: u64) {
		self.hasher = Xxh3Hash64::with_seed(seed);
	}

	pub fn update_string(&mut self, data: String) {
		self.hasher.write(data.as_bytes());
	}

	pub fn update_bytes(&mut self, data: &[u8]) {
		self.hasher.write(data);
	}

	pub fn digest(&self) -> u64{
		self.hasher.finish()
	}
}