# Demo C++ app that statically links with a Rust lib

## Prerequisites for Ubuntu 20.04
* `sudo apt install build-essential cmake cargo`
* `cargo install cbindgen`

## Build steps
1.  `cd build`
2. `cmake ../ -DCMAKE_BUILD_TYPE=Release`
3. `make -j`

The above commands will build the Rust lib project inside `librtest` and then build the C++ app inside `src`.
* To execute the final linked binary run `./interopcc`

