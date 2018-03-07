
use gleam::gl;
use gleam::gl::{GLenum, GLuint};
use std;

type GlPtr = std::rc::Rc<gl::Gl>;

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