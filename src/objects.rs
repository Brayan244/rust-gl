use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};

use gl::{
    types::{GLchar, GLenum, GLint, GLuint},
    UseProgram,
};

// Helper function to create a CString with whitespace
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

// Shader struct and its implementation
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        unsafe {
            let id: u32 = gl::CreateShader(kind);
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);

            let mut success: GLint = 1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut error_message_length: GLint = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut error_message_length);

                let error_message: CString =
                    create_whitespace_cstring_with_len(error_message_length as usize);

                gl::GetShaderInfoLog(
                    id,
                    error_message_length,
                    null_mut(),
                    error_message.as_ptr() as *mut GLchar,
                );

                return Err(error_message.to_string_lossy().into_owned());
            }

            Ok(Shader { id })
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

// Program struct and its implementation
pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        unsafe {
            let id: GLuint = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader(id, shader.id());
            }

            gl::LinkProgram(id);

            let mut success: GLint = 1;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut error_message_length: GLint = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut error_message_length);

                let error_message: CString =
                    create_whitespace_cstring_with_len(error_message_length as usize);

                gl::GetProgramInfoLog(
                    id,
                    error_message_length,
                    null_mut(),
                    error_message.as_ptr() as *mut GLchar,
                );

                return Err(error_message.to_string_lossy().into_owned());
            }

            Ok(Program { id })
        }
    }

    pub fn set(&self) {
        unsafe {
            UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

// Function to create a program
pub fn create_program() -> Result<Program, Box<dyn std::error::Error>> {
    let vertex_shader = load_shader_from_file("./src/shaders/vertex.glsl", gl::VERTEX_SHADER)?;
    let fragment_shader =
        load_shader_from_file("./src/shaders/fragment.glsl", gl::FRAGMENT_SHADER)?;
    let program = Program::from_shaders(&[vertex_shader, fragment_shader])?;
    Ok(program)
}

// Function to load a shader from a file
fn load_shader_from_file(
    filename: &str,
    kind: GLenum,
) -> Result<Shader, Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(filename)?;
    let c_str = CString::new(source.as_bytes())?;
    let shader = Shader::from_source(&c_str, kind)?;
    Ok(shader)
}

/// OpenGL Vertex Buffer Object
pub struct Vbo {
    pub id: GLuint,
}

impl Drop for Vbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Vbo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Vbo { id }
    }

    pub fn set(&self, data: &Vec<f32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, vertices: &Vec<f32>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

/// OpenGL Index Buffer Object
pub struct Ibo {
    pub id: GLuint,
}

impl Drop for Ibo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Ibo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Ibo { id }
    }

    pub fn set(&self, data: &Vec<u32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

/// OpenGL Vertex Array Object
pub struct Vao {
    pub id: GLuint,
}

impl Drop for Vao {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

impl Vao {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Vao { id }
    }

    pub fn set(&self) {
        self.bind();
        self.setup();
    }

    fn setup(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as GLint,
                null(),
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
