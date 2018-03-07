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
        , "DeleteQueriesEXT"
        , "DeleteSamplers"
        , "DeleteSync"
        , "DeleteTransformFeedbacks"
        , "EGLImageTargetRenderbufferStorageOES"
        , "EGLImageTargetTexture2DOES"
        , "EndQuery"
        , "EndQueryEXT"
        , "EndTransformFeedback"
        , "FenceSync"
        , "FlushMappedBufferRange"
        , "FramebufferTextureLayer"
        , "GenQueries"
        , "GenQueriesEXT"
        , "GenSamplers"
        , "GenTransformFeedbacks"
        , "GetActiveUniformBlockName"
        , "GetActiveUniformBlockiv"
        , "GetActiveUniformsiv"
        , "GetBufferParameteri64v"
        , "GetBufferPointerv"
        , "GetFragDataLocation"
        , "GetInteger64i_v"
        , "GetInteger64v"
        , "GetIntegeri_v"
        , "GetInternalformativ"
        , "GetProgramBinary"
        , "GetQueryObjecti64vEXT"
        , "GetQueryObjectivEXT"
        , "GetQueryObjectui64vEXT"
        , "GetQueryObjectuiv"
        , "GetQueryObjectuivEXT"
        , "GetQueryiv"
        , "GetQueryivEXT"
        , "GetSamplerParameterfv"
        , "GetSamplerParameteriv"
        , "GetStringi"
        , "GetSynciv"
        , "GetTransformFeedbackVarying"
        , "GetUniformBlockIndex"
        , "GetUniformIndices"
        , "GetUniformuiv"
        , "GetVertexAttribIiv"
        , "GetVertexAttribIuiv"
        , "InsertEventMarkerEXT"
        , "InvalidateFramebuffer"
        , "InvalidateSubFramebuffer"
        , "IsQuery"
        , "IsQueryEXT"
        , "IsSampler"
        , "IsSync"
        , "IsTransformFeedback"
        , "MapBufferRange"
        , "PauseTransformFeedback"
        , "PopGroupMarkerEXT"
        , "ProgramBinary"
        , "ProgramParameteri"
        , "PushGroupMarkerEXT"
        , "QueryCounterEXT"
        , "ReadBuffer"
        , "RenderbufferStorageMultisample"
        , "ResumeTransformFeedback"
        , "SamplerParameterf"
        , "SamplerParameterfv"
        , "SamplerParameteri"
        , "SamplerParameteriv"
        , "TexImage3D"
        , "TexSubImage3D"
        , "TexStorage2D"
        , "TexStorage3D"
        , "TransformFeedbackVaryings"
        , "Uniform1ui"
        , "Uniform1uiv"
        , "Uniform2ui"
        , "Uniform2uiv"
        , "Uniform3ui"
        , "Uniform3uiv"
        , "Uniform4ui"
        , "Uniform4uiv"
        , "UniformBlockBinding"
        , "UniformMatrix2x3fv"
        , "UniformMatrix2x4fv"
        , "UniformMatrix3x2fv"
        , "UniformMatrix3x4fv"
        , "UniformMatrix4x2fv"
        , "UniformMatrix4x3fv"
        , "UnmapBuffer"
        , "VertexAttribI4i"
        , "VertexAttribI4iv"
        , "VertexAttribI4ui"
        , "VertexAttribI4uiv"
        , "VertexAttribIPointer"
        , "WaitSync"
    ];

    let mut gles_reg = Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, gles_extensions);

    let mut filtered : BTreeSet<Cmd> = BTreeSet::new();

    gles_reg.cmds.iter().for_each(|item|{
        let ident = &item.proto.ident;
        let cs:&str = &ident;
        //println!("{:?}", ident);

        if exclude.contains(&cs) {
        }
        else {
            //println!("{:?}", &cs);
            filtered.insert(Cmd::clone(item));
        }
    });

    gles_reg.cmds = filtered;

    gles_reg.write_bindings(gl_generator::StructGenerator, &mut file_gles)
        .unwrap();

    // OpenGL 3.3 + GLES 3.0 bindings. Used to get all enums
    let gl_reg = gl_reg + gles_reg;
    gl_reg.write_bindings(gl_generator::StructGenerator, &mut file_gl_and_gles)
        .unwrap();
}