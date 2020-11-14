# twoxhash-wasm

A WebAssembly implementation of the xxhash hashing algorithm. Written in Rust using the twox-hash crate and wasm-bindgen.

Using WebAssembly doesn't mean you have to give up the nice things Javascript has to offer. twoxhash-wasm offload the heavy computation to WebAssembly while you continue to write Javascript!

## Usage
The main bundle and WebAssembly code are loaded in only when needed, thus an initialization function needs to be called before any additional functionalities are available. Everything are scoped under the `twoxhash` namespace.

```javascript
// The `init()` function returns a promise when all external modules are loaded
twoxhash.init().then(() => {
	// Create a 64-bit xxhash hasher instance
	const hasher = new twoxhash.XXHash64();

	// Simple one line hash, this is always independent of hasher's internal state
	let result = hasher.hash("Hello, world!"); // f58336a78b6f9476

	// Initialize hasher with a seed
	hasher.init(42);
	// Update internal state of hasher
	hasher.update("One");
	hasher.update("Two");
	hasher.update("Three");
	// Get hash result
	result = hasher.digest(); // 76cfb4ada0be05cd
	// The hasher can be continually updated with data even after calling `digest()`
	// Call `hasher.init()` again to reset hasher's internal state
});
````

Available hasher constructors are:
- `XXHash32`
- `XXHash64`
- `XXHash3` - xxhash3 with 64-bit output
- `XXHash128` - xxhash3 with 128-bit output

All hasher instances regardless of constructors have the same methods:
- `hasher.init([seed: number])`: Initialize the hasher with an optional seed value. wasm-bindgen converts the input value into a u64, truncation may occur near upper limit of possible value.
- `hasher.update(value: String|Uint8Array)`: Update internal state of the hasher with provided value. Both `String` or `Uint8Array` can be passed in, more value types may be added in the future.
- `hasher.digest()`: Returns the value of the hash at the hasher's current state. Does not reset the hasher. Result returned as a base 16 string, more flexible ways to return data may be added in the future.
- `hasher.hash(value: String|Uint8Array, [seed: number])`: One off hashing that does not use nor alter the internal state of the hasher. Optional seed can be provided the same way as `hasher.init()`. Result returned the same wat as `hasher.digest()`.

## Future
I had fun coding this up and still have plenty of ideas about extending it! The ultimate aim is to create an easy to use interface while leveraging the performance of WebAssembly.

This project does not have lofty claims of having the smallest package size or the fastest implementation, while it does take those into account, the point is to make something fast while easy to use. There's no point in creating the smallest bundle size if the code is spaghetti all the way down, that's code golf. There's also no point in creating the fastest bundle size if it feels like you are using C/C++/Rust or some other programming language, I'll leave that exercise to someone else.
