
use gleam::gl;
use gleam::gl::{GLenum, GLuint};
use std;

use emscripten_sys::{
    emscripten_fetch_t,
    emscripten_fetch_attr_t,
    emscripten_fetch_attr_init,
    emscripten_fetch,
    emscripten_fetch_close
};

use serde_json;

type GlPtr = std::rc::Rc<gl::Gl>;



/** Original Example START */
pub fn load_shader(gl: &GlPtr, shader_type: GLenum, source: &[&[u8]]) -> Option<GLuint> {
    let shader = gl.create_shader(shader_type);
    if shader == 0 {
        return None;
    }
    gl.shader_source(shader, source);
    gl.compile_shader(shader);
    let compiled = gl.get_shader_iv(shader, gl::COMPILE_STATUS);
    if compiled == 0 {
        let log = gl.get_shader_info_log(shader);
        println!("{}", log);
        gl.delete_shader(shader);
        return None;
    }
    Some(shader)
}


pub const VS_SRC: &'static [&[u8]] = &[b"#version 300 es
        layout(location = 0) in vec3 aPosition;
        layout(location = 1) in vec3 aColor;
        uniform mat4 uMVMatrix;
        uniform mat4 uPMatrix;
        out vec4 vColor;
        void main() {
            gl_Position = uPMatrix * uMVMatrix * vec4(aPosition, 1.0);
            vColor = vec4(aColor, 1.0);
        }"];

pub const FS_SRC: &'static [&[u8]] = &[b"#version 300 es
        precision mediump float;
        in vec4 vColor;
        out vec4 oFragColor;
        void main() {
            oFragColor = vColor;
        }"];

/** Original Example END */

/** https://qiita.com/_likr/items/71817953b237a32a3015 */
pub fn body_string(fetch: &emscripten_fetch_t) -> String {
    let data = unsafe { std::mem::transmute::<*const i8, *mut u8>((*fetch).data) };
    let len = (*fetch).totalBytes as usize;
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    let mut v = Vec::with_capacity(len);
    v.resize(len, 0);
    v.copy_from_slice(slice);
    String::from_utf8(v).ok().unwrap()
}

fn print_json(fetch: &emscripten_fetch_t) {
    let body = body_string(fetch);
    match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(obj) => {
            println!("{}", obj["a"]);
        }
        Err(e) => {
            println!("error: line: {}, column: {}", e.line(), e.column());
        }
    }
}

pub extern "C" fn handle_success(fetch: *mut emscripten_fetch_t) {
    unsafe {
        print_json(&*fetch);
        emscripten_fetch_close(fetch);
    }
}

pub extern "C" fn handle_error(fetch: *mut emscripten_fetch_t) {
    unsafe {
        println!("error: status code {}", (*fetch).status);
        emscripten_fetch_close(fetch);
    }
}
























