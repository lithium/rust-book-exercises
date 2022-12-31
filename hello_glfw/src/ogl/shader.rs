use gl;
use std;
use gl::types::*;
use std::ptr;
use std::ffi::{CString, CStr};

use crate::ogl;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn from_source(source: &str, shader_type: GLenum) -> Result<Shader, String> {
        let csource = CString::new(source).unwrap();
        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(id, 1, &csource.as_ptr(), ptr::null());
            gl::CompileShader(id);
        };

        let mut success = gl::FALSE as GLint;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == gl::TRUE as GLint {
            return Ok(Shader {id});
        } else {
            let mut len = 0 as GLint;
            unsafe { 
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = ogl::create_cstr(len as usize);
            unsafe {
                gl::GetShaderInfoLog(id, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }
            return Err(error.to_string_lossy().into_owned());
        }

    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

