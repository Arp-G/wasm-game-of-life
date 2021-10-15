
## Learning WASM from the [Rust WASM Book](https://rustwasm.github.io/docs/book/introduction.html)

This readme contains some key points that I summarized.

### Getting started with a simple wasm project

* Clone the project: cargo generate --git https://github.com/rustwasm/wasm-pack-template
* Build the wasm `pkg` to be used from javascript via npm using `wasm-pack build`
* Initialize node wasm project `npm init wasm-app www`
* Update `package.json` with wasm dependency

  ```
   "dependencies": {
    "wasm-game-of-life": "file:../pkg"
  }
  ```
* Import wasm dependency and call wasm functions

  ```
  import * as wasm from "wasm-game-of-life";


* Rebuild wasm project every time you make changes to wasm code using `wasm-pack build`


### JS vs Rust memory management

JavaScript's garbage-collected heap — where Objects, Arrays, and DOM nodes are allocated — is distinct from WebAssembly's linear memory space, where our Rust values live. WebAssembly currently has no direct access to the garbage-collected heap (as of April 2018, this is expected to change with the "Interface Types" proposal). 

JavaScript, on the other hand, can read and write to the WebAssembly linear memory space, but only as an ArrayBuffer of scalar values (u8, i32, f64, etc...). WebAssembly functions also take and return scalar values.

In other words...

Each wasm module has a linear memory, which is initialized during instantiation. JS code can freely read and write to this memory.

By contrast, wasm code has no direct access to JS objects.

### A good JavaScript↔WebAssembly interface

* Minimizing copying into and out of the WebAssembly linear memory and Minimizing serializing and deserializing

* Large, long-lived data structures are implemented as Rust types that live in the WebAssembly linear memory
  , and are exposed to JavaScript as opaque handles(like a raw pointer). 
  
* JavaScript calls exported WebAssembly functions that take these opaque handles, transform their data, perform heavy computations, query the data, and ultimately return a small, copy-able result.

* Since wasm code does not have direct access to javascript heap memory, to manipulate such memory wasm can invoke imported JS functions.

Learn more [here](https://rustwasm.github.io/docs/book/reference/js-ffi.html#javascript-interoperation)

### Debugging

* Enable debug symbols, this preserves function names in the compiled .wasm binary and therfore in the stack traces otherwise so stack traces instead of names like `wasm-function[42]`, debugging in not present in rust release builds to enable it set `debug = true` under `[profile.release]` in `Cargo.toml`

* Log rust panics to browser console via the `console_error_panic_hook` hook
  This crate is by default installed and setup when using the `wasm-pack-template`

  Just initialize the hook at a common code path like `utils::set_panic_hook();`

* To log custom messages to browser console, install the `web.sys` crate 
  and enable the `console` feature on it by...
  ```
  [dependencies.web-sys]
  version = "0.3"
  features = [ "console"]
  ```

  Now you can log to browser console like...
  ```
  extern crate web_sys;
  web_sys::console::log_1(&"Hello, world!".into());
  ``` 
 
  (Optional: Make the logger a macro for easy usage)

### Profiling

* By preserving rust function names use the browsers profiler(better uyse firefox) to identify performance bottelnecks.

* Avoid expensive operations like clonning memory, etc if possible.

* Use the js functions `window.performance.now`, `console.time` and `console.timeEnd`
  These can be executed from Rust using the `web_sys` crate like for example...
  ```
  extern crate web_sys;
  use web_sys::console;

  console::time_with_label("some identifier");
  ```

  You can check how much time any block of code in rust took by wrapping the code in a block and
  initializing an object that calls `console.time`. This object will get dropped after block scope ends, 
  on drop the object will call `console.timeEnd` this can be achieved by implementing the `Drop` trait.
  Look [here](https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html#time-each-universetick-with-consoletime-and-consoletimeend) for details.

* Perform [benchmarking](https://doc.rust-lang.org/unstable-book/library-features/test.html#test) this can be done using the `#[bench]` annotation.

* When running bench it generates a binary, use the [`perf` performnace analysis tool in linux](https://www.youtube.com/watch?v=M6ldFtwWup0) to analyze performance of the binary and you can look into assembly code level calls and commands which might be bottlenecks.

  Example:
  ```
  cargo bench | tee before.txt
  perf record -g target/release/deps/bench-8474091a05cfa2d9 --bench
  perf report
  ```

### Reducing wasm size
[Details](https://rustwasm.github.io/docs/book/reference/code-size.html#use-the-wasm-opt-tool]

* Optimizing Builds for Code Size by specifying options to the compiler via like
  ```
  [profile.release]
  opt-level = "z"
  lto = true
  ```
  (Compiling with Link Time Optimizations (LTO))
  (opt-level = "z" means aggressively optimize for size)

* Use the [wasm-opt-tool](https://rustwasm.github.io/docs/book/reference/code-size.html#use-the-wasm-opt-tool)

### Some Points

* Any function that is to be exported to js world should be annoted by the `#[wasm_bindgen]` attribute

* Rust-generated WebAssembly functions cannot return borrowed references but you can return raw pointers to memory locations like...
  `pub fn cells(&self) -> *const Cell {  self.cells.as_ptr()  }`

* Under the hood how wasm code is streamed to js environment and executed
  https://www.hellorust.com/demos/add/index.html
  https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/instantiateStreaming
