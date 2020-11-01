let wasm;
async function init(){
	wasm = await import("xxhash-wasm");
}

class Hasher {
	constructor(hashConstructor, seed=0){
		this.hashConstructor = hashConstructor;
	}

	hash(data, seed=0){
		if(typeof data === "string"){
			return this.hasher.hash_string(data, seed);
		}else if(data.constructor === Uint8Array){
			return this.hasher.hash_bytes(data, seed);
		}else{
			console.error("Invalid data type. Only String and Uint8Array accepted");
		}
	}

	update(data){
		if(typeof data === "string"){
			this.hasher.update_string(data);
		}else if(data.constructor === Uint8Array){
			this.hasher.update_bytes(data);
		}else{
			console.error("Invalid data type. Only String and Uint8Array accepted");
		}
	}

	digest(){
		return this.hasher.digest();
	}
}

class XXHash32 extends Hasher {
	constructor(seed=0){
		super(wasm.Hash32, seed);
		this.hasher = this.hashConstructor.new(seed);
	}

	init(seed=0){
		this.hasher = this.hashConstructor.new(seed);
	}
}

class XXHash64 extends Hasher {
	constructor(seed=0){
		super(wasm.Hash64, seed);
		this.hasher = this.hashConstructor.new(BigInt(seed));
	}

	init(seed=0){
		this.hasher = this.hashConstructor.new(BigInt(seed));
	}
}

class XXHash3 extends Hasher {
	constructor(seed=0){
		super(wasm.Hash64, seed);
		this.hasher = this.hashConstructor.new(BigInt(seed));
	}

	init(seed=0){
		this.hasher = this.hashConstructor.new(BigInt(seed));
	}
}

// Not exported as wasm-bidgen cannot output u128 yet
class XXHash128 extends Hasher {
	constructor(seed=0){
		super(wasm.Hash64, seed);
		this.hasher = this.hashConstructor.new(BigInt(seed));
	}

	init(seed=0){
		this.hasher = this.hashConstructor.new(BigInt(seed));
	}
}

export {
	init,
	XXHash32,
	XXHash64,
	XXHash3
};