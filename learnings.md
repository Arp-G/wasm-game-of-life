
### Getting started

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

  wasm.greet();
  ```

* Rebuild wasm project every time you make changes to wasm code using `wasm-pack build`


### JS vs Rust memory management

JavaScript's garbage-collected heap — where Objects, Arrays, and DOM nodes are allocated — is distinct from WebAssembly's linear memory space, where our Rust values live. WebAssembly currently has no direct access to the garbage-collected heap (as of April 2018, this is expected to change with the "Interface Types" proposal). JavaScript, on the other hand, can read and write to the WebAssembly linear memory space, but only as an ArrayBuffer of scalar values (u8, i32, f64, etc...). WebAssembly functions also take and return scalar values.

### A good JavaScript↔WebAssembly interface

* Minimizing copying into and out of the WebAssembly linear memory and Minimizing serializing and deserializing

* Large, long-lived data structures are implemented as Rust types that live in the WebAssembly linear memory
  , and are exposed to JavaScript as opaque handles. 
  
* JavaScript calls exported WebAssembly functions that take these opaque handles, transform their data, perform heavy computations, query the data, and ultimately return a small, copy-able result.