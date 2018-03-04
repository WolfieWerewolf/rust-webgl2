extern crate gleam;
extern crate emscripten_sys;

mod matrix;
mod context;

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
};




fn main() {

//    let excluded = vec2(
//        "glClearBufferfv",
//        "glClearBufferiv"
//    );

    unsafe {
        let mut attributes: EmscriptenWebGLContextAttributes = std::mem::uninitialized();

        emscripten_webgl_init_context_attributes(&mut attributes);

        attributes.majorVersion = 2;

        let handle = emscripten_webgl_create_context(std::ptr::null(), &attributes);

        emscripten_webgl_make_context_current(handle);


        let gl = gl::GlesFns::load_with(|addr| {
            let addr = std::ffi::CString::new(addr).unwrap();

            emscripten_GetProcAddress(addr.as_ptr() as *const _) as *const _

        });

//        let gl = gl::GlFns::load_with(|addr| {
//            let addr = std::ffi::CString::new(addr).unwrap();
//            emscripten_GetProcAddress(addr.into_raw() as *const _) as *const _
//        });


        let mut ctx = Context::new(gl);
        let ptr = &mut ctx as *mut _ as *mut std::os::raw::c_void;
        emscripten_set_main_loop_arg(Some(loop_wrapper), ptr, 0, 1);
    }
}
