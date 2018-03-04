# rust-webgl2

This project is based on:

Rust + WebAssembly + WebGL 2.0 Demo

https://github.com/likr/rust-webgl2-example

The original demo can be found here:

https://likr.github.io/rust-webgl2-example


# How to build
I'm using Debian 9 and followed this procedure to get up and running with
rust and Emscripten.  

https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627

As of today (March 4 / 2018) I did _not_ have to use Rust nightly.

What I did have to do, however, is add the following to ~/.bashrc

```console
export PATH="$HOME/emsdk-portable/emscripten/1.37.35:$PATH"
```

and I find it necessary to call:
```console
$ source path/to/emsdk/emsdk_env.sh
```
from each bash session.. I haven't tried adding that to the script yet.

```console
$ source path/to/emsdk/emsdk_env.sh
$ cargo build --release --target=wasm32-unknown-emscripten
```

You can also build for asm.js like this:

```console
$ source path/to/emsdk/emsdk_env.sh
$ cargo build --release --target=asmjs-unknown-emscripten
```

but you will have to comment the wasm related call and uncomment the asmjs related call in
the index.html script stags.


