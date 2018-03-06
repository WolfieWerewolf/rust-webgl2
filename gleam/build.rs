/** Borrowed from [Documentation](http://doc.servo.org/gleam/) */
extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;
use gl_generator::{Registry, Api, Profile, Fallbacks, Cmd};
use std::collections::{BTreeSet};

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file_gl_and_gles = File::create(&Path::new(&dest).join("gl_and_gles_bindings.rs")).unwrap();
    let mut file_gl = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();
    let mut file_gles = File::create(&Path::new(&dest).join("gles_bindings.rs")).unwrap();

    // OpenGL 3.3 bindings
    let gl_extensions = ["GL_ARB_texture_rectangle",
        "GL_EXT_debug_marker",
        "GL_APPLE_client_storage",
        "GL_APPLE_texture_range",
        "GL_APPLE_fence",
        "GL_ARB_get_program_binary",
        "GL_ARB_blend_func_extended"];
    let gl_reg = Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, gl_extensions);
    gl_reg.write_bindings(gl_generator::StructGenerator, &mut file_gl)
        .unwrap();

    // GLES 3.0 bindings
    let gles_extensions = [
        "GL_EXT_texture_format_BGRA8888",
        "GL_OES_EGL_image",
        "GL_OES_EGL_image_external",
        "GL_EXT_disjoint_timer_query",
        "GL_EXT_debug_marker",
    ];

    let exclude = [
        "BeginQuery"
        , "BeginQueryEXT"
        , "BeginTransformFeedback"
        , "BindSampler"
        , "BindBufferBase"
        , "BindBufferRange"
        , "BindBufferRange"
        , "BindTransformFeedback"
        , "BlitFramebuffer"
        , "ClearBufferfi"
        , "ClearBufferfv"
        , "ClearBufferiv"
        , "ClearBufferuiv"
        , "ClientWaitSync"
        , "CompressedTexImage3D"
        , "CompressedTexSubImage3D"
        , "CopyBufferSubData"
        , "CopyTexSubImage3D"
        , "DeleteQueries"
    ];

    let mut gles_reg = Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, gles_extensions);

    let mut filtered : BTreeSet<Cmd> = BTreeSet::new();

    gles_reg.cmds.iter().for_each(|item|{
        let ident = &item.proto.ident;
        let cs:&str = &ident;
        //println!("{:?}", ident);

        if exclude.contains(&cs) {
            //println!("{:?}","Excluded --------------------");
            //println!("{:?}", &cs);
        }
        else {
            //println!("{:?}","ATTEMPTED --------------------");
            println!("{:?}", &cs);
            filtered.insert(Cmd::clone(item));
        }
    });

    //gles_reg.cmds.clear();

    gles_reg.cmds = filtered;





    gles_reg.write_bindings(gl_generator::StructGenerator, &mut file_gles)
        .unwrap();

    // OpenGL 3.3 + GLES 3.0 bindings. Used to get all enums
    let gl_reg = gl_reg + gles_reg;
    gl_reg.write_bindings(gl_generator::StructGenerator, &mut file_gl_and_gles)
        .unwrap();
}

//gles_reg.cmds = gles_reg.get_tys().iter().take_while(|item| !exclude.contains(&item) );

//let filtered = gles_reg.get_tys().iter().take_while(|item| !exclude.contains(&item) );

//    gles_reg.enums.iter().for_each(|item|{
//        println!("{:?}", item)
//    });




//    gles_reg.cmds.iter().for_each(|item|{
//        let ident = &item.proto.ident;
//        let cs:&str = &ident;
//        if exclude.contains(&cs) {
//            tys.insert
//            //    gles_reg.cmds.remove(&item);
//            //println!("{:?}", ident)
//        }
//        //println!("{:?}", ident)
//    });

//    pub fn get_tys(reg: Registry) -> BTreeSet<&str> {
//        let mut tys = BTreeSet::new();
//        for def in &reg.cmds {
//
//            let ident = &def.proto.ident;
//            let cs:&str = &ident;
//
//            tys.insert(def.proto.ty.as_ref());
//            for param in &def.params {
//                tys.insert(param.ty.as_ref());
//            }
//
////            if !exclude.contains(&cs) {
////                tys.insert(def.proto.ty.as_ref());
////                for param in &def.params {
////                    tys.insert(param.ty.as_ref());
////                }
////            }
//        }
//        tys
//    }

//exclude.iter().for_each(|item| {
//let ident = &item.proto.ident;
//let cs:&str = &ident;
//if  gles_reg.cmds.contains(&cs) {
//gles_reg.cmds.remove(&item.proto.ident);
//}
//println!("{:?}", ident)
//});
