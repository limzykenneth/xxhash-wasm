use super::utils;
use wasm_bindgen::prelude::*;
use twox_hash::Xxh3Hash128;
use std::hash::Hasher;
use twox_hash::xxh3::HasherExt;

#[wasm_bindgen]
pub struct Hash128 {
	hasher: Xxh3Hash128
}

#[wasm_bindgen]
impl Hash128 {
	pub fn new(seed: u64) -> Hash128 {
		utils::set_panic_hook();

		Hash128 {
			hasher: Xxh3Hash128::with_seed(seed)
		}
	}

	pub fn hash_string(&self, data: String, seed: JsValue) -> Result<String, JsError> {
		let mut hasher;

		if seed.is_undefined() {
			hasher = Xxh3Hash128::with_seed(0);
		} else {
			let seed_val = match seed.as_f64() {
				Some(x) => x as u64,
				None => return Err(JsError::new("Seed must be a number")),
			};
			hasher = Xxh3Hash128::with_seed(seed_val);
		}

		hasher.write(data.as_bytes());
		Ok(format!("{:x}", hasher.finish_ext()))
	}

	pub fn hash_bytes(&self, data: &[u8], seed: JsValue) -> Result<String, JsError> {
		let mut hasher;

		if seed.is_undefined() {
			hasher = Xxh3Hash128::with_seed(0);
		} else {
			let seed_val = match seed.as_f64() {
				Some(x) => x as u64,
				None => return Err(JsError::new("Seed must be a number")),
			};
			hasher = Xxh3Hash128::with_seed(seed_val);
		}

		hasher.write(data);
		Ok(format!("{:x}", hasher.finish_ext()))
	}

	pub fn init(&mut self, seed: u64) {
		self.hasher = Xxh3Hash128::with_seed(seed);
	}

	pub fn update_string(&mut self, data: String) {
		self.hasher.write(data.as_bytes());
	}

	pub fn update_bytes(&mut self, data: &[u8]) {
		self.hasher.write(data);
	}

	pub fn digest(&self) -> String {
		format!("{:x}", self.hasher.finish_ext())
	}
}