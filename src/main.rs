extern crate gleam;
extern crate emscripten_sys;
extern crate plain_enum;
extern crate serde_json;

mod matrix;
mod context;
mod shader_loader;

use shader_loader::{
    handle_success,
    handle_error
};

use context::{
    Context,
    loop_wrapper
};

use gleam::gl;

use emscripten_sys::{
    emscripten_set_main_loop_arg,
    emscripten_GetProcAddress,
    emscripten_webgl_init_context_attributes,
    emscripten_webgl_create_context,
    emscripten_webgl_make_context_current,
    EmscriptenWebGLContextAttributes,
    emscripten_fetch_t,
    emscripten_fetch_attr_t,
    emscripten_fetch_attr_init,
    emscripten_fetch,
    emscripten_fetch_close,
};

#[link(name = "_hello_world")]
extern {}

#[no_mangle]
pub extern fn hello_world(n: std::os::raw::c_int) -> std::os::raw::c_int {
    n + 1
}

mod example_enum {
    use plain_enum::*;
    plain_enum_mod!{example_enum, ExampleEnum {
        BUFFER,
        FRAMEBUFFER,
        PROGRAM,
        RENDERBUFFER,
        SHADER,
        TEXTURE, // note trailing comma
    }}
    pub fn do_stuff(){
        for value in ExampleEnum::values() {            // iterating over the enum's values
            println!("{:?}", value);
        }
    }
}

fn main() {

    example_enum::do_stuff();
    //for value in example_enum::ExampleEnum::values() {            // iterating over the enum's values
    //    println!("{:?}", value);
    //}

    /** Fetch API Test START */
    unsafe {
        let mut fetch_arg: emscripten_fetch_attr_t = std::mem::uninitialized();
        emscripten_fetch_attr_init(&mut fetch_arg);
        fetch_arg.attributes = 1;
        fetch_arg.onsuccess = Some(handle_success);
        fetch_arg.onerror = Some(handle_error);
        let url = std::ffi::CString::new("js/data.json").unwrap();
        emscripten_fetch(&mut fetch_arg, url.as_ptr());
    }
    /** Fetch API Test END */


    unsafe {
        let mut attributes: EmscriptenWebGLContextAttributes = std::mem::uninitialized();

        emscripten_webgl_init_context_attributes(&mut attributes);

        attributes.majorVersion = 2;

        let handle = emscripten_webgl_create_context(std::ptr::null(), &attributes);

        emscripten_webgl_make_context_current(handle);

        // We need this pull request: https://github.com/kripken/emscripten/pull/4829
        let gl = gl::GlesFns::load_with(|addr| {
            let addr = std::ffi::CString::new(addr).unwrap();
            emscripten_GetProcAddress(addr.into_raw() as *const _) as *const _
        });

        let mut ctx = Context::new(gl);
        let ptr = &mut ctx as *mut _ as *mut std::os::raw::c_void;
        emscripten_set_main_loop_arg(Some(loop_wrapper), ptr, 0, 1);
    }
}


