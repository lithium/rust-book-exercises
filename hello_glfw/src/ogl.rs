pub mod ogl {
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
            let id = compile_shader(&csource, shader_type)?;
            Ok(Shader {id})
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe { gl::DeleteShader(self.id); }
        }
    }

    fn compile_shader(source: &CStr, shader_type: GLenum) -> Result<GLuint, String> {
        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(id);
        };

        let mut success = gl::FALSE as GLint;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == gl::TRUE as GLint {
            return Ok(id);
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

    fn create_cstr(len: usize) -> CString {
        let mut buffer: Vec<u8> = Vec::with_capacity(len+1);
        buffer.extend([b' '].iter().cycle().take(len));
        unsafe { CString::from_vec_unchecked(buffer) }
    }
}
