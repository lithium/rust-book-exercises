use gl;
use std;
use gl::types::*;
use std::ptr;
use std::ffi::{CString, CStr};



pub struct Shader {
    id: GLuint,
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
            let error = create_cstr(len as usize);
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




pub struct Program {
    id: GLuint,
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
            let error = create_cstr(len as usize);
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



fn create_cstr(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len+1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
