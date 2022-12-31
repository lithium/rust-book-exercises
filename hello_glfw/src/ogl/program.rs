use gl;
use std;
use gl::types::*;
use std::ptr;
use std::ffi::{CString, CStr};

use crate::ogl;
use crate::ogl::shader::Shader;

pub struct Program {
    pub id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id); }
        }

        unsafe { gl::LinkProgram(id); }

        let mut success = gl::FALSE as GLint;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success != gl::TRUE as GLint {
            let mut len = 0 as GLint;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = ogl::create_cstr(len as usize);
            unsafe {
                gl::GetProgramInfoLog(id, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }
            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(id, shader.id); }
        }

        Ok(Program {id})
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}



