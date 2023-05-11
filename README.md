# Rust JSON Parser
This repository contains a Rust library that provides a simple function for parsing JSON strings, which can be called from Node.js using FFI.

## Prerequisites
 - Rust: https://www.rust-lang.org/tools/install
 - Node.js: https://nodejs.org/en/download/
## Installation
1. Clone this repository:

```js
git clone https://github.com/auralshin/rust_json_parser.git
cd rust-ffi-json-parsing-example
```
2. Build the Rust library:
```js
cargo build --release
```
This will create a dynamic library file (**libjson_parser.so** on Linux, **libjson_parser.dylib** on macOS, or **json_parser.dll** on Windows) in the **target/release** directory.

3. Install the Node.js dependencies:

```js
cd node
npm install
```
## Usage
To use the Rust library from Node.js, you'll need to load the dynamic library file and define a function that wraps the Rust function. Here's an example:
```js
const path = require('path');
const ffi = require('ffi');

// Load the rust_json_parser library from crates.io
const lib = ffi.Library('librust_json_parser', {
  parse_json: ['string', ['string']]
});


// Define a wrapper function that calls the Rust function
function parseJSON(jsonStr) {
  const result = lib.parse_json(jsonStr);
  if (!result) {
    throw new Error('Failed to parse JSON');
  }
  return result;
}

// Usage example
const jsonStr = '{"name": "Alice", "age": 30}';
const parsedJSON = parseJSON(jsonStr);
console.log(parsedJSON);

```
This code loads the libjson_parser library using the **ffi** module, and defines a **parseJSON()** function that wraps the **parse_json()** function provided by the Rust library. To use the **parseJSON()** function, simply pass a JSON string as an argument, and it will return a parsed JSON string on success, or throw an error on failure.

## License
This code is licensed under the MIT License. Feel free to use it as a starting point for your own projects!
