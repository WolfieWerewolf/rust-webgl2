


use std::any::Any;

//pub enum ResType {
//    BUFFER,
//    FRAMEBUFFER,
//    PROGRAM,
//    RENDERBUFFER,
//    SHADER,
//    TEXTURE
//}




pub struct GLResource {
    //gl_ref: Any,
    is_valid: bool,
    delete: fn() -> ()
}

