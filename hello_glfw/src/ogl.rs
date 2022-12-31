use std;
use std::ffi::CString;


pub mod shader;
pub use self::shader::Shader;


pub mod program;
pub use self::program::Program;


pub fn create_cstr(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len+1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
