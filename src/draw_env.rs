use gl::types::*;

use super::handles::*;
use super::glsl_type::*;

pub(super) enum EnableDepth {
	No,
	Yes(GLenum),
}

pub(super) enum EnableBlend {
	No,
	Yes(GLenum, GLenum),
}

// :)
pub struct DrawEnv {
	// core stuff
	pub(super) count: usize,
	pub(super) offset: usize,
	pub(super) draw_type: GLenum,
	pub(super) shader: usize,
	pub(super) indices: usize,
	pub(super) buffers: Vec<(usize, GLuint)>,
	pub(super) textures: Vec<(usize, GLint)>,
	pub(super) uniforms: Vec<(GLSLAny, GLint)>,

	// optional stuff
	pub(super) depth: EnableDepth,
	pub(super) blend: EnableBlend,
}

impl DrawEnv {
	pub(super) fn new() -> Self {
		Self {
			count: 0,
			offset: 0,
			draw_type: 0,

			shader: usize::max_value(),
			indices: usize::max_value(),
			buffers: Vec::new(),
			textures: Vec::new(),
			uniforms: Vec::new(),

			depth: EnableDepth::No,
			blend: EnableBlend::No,
		}
	}

	pub fn debug_print(&self) {
		println!("Printing DrawEnv:");
		println!("count: {}, offset: {}", self.count, self.offset);
		println!("shader: {}", self.shader);
		println!("ibo: {}", self.indices);
		println!("Buffers:");
		self.buffers.iter().for_each(|&(h, b)| {
			println!("  {}, target {}", h, b);
		});
		println!("Textures:");
		self.textures.iter().for_each(|&(h, b)| {
			println!("  {}, target {}", h, b);
		});
		println!("Uniforms:");
		self.uniforms.iter().for_each(|&(ref a, b)| {
			println!("  {:?}, target {}", a, b);
		});
		println!("End print");
	}

	pub fn set_draw_count(&mut self, count: usize, offset: usize, draw_type: GLenum) {
		self.count = count;
		self.offset = offset;
		self.draw_type = draw_type;
	}

	pub fn add_shader(&mut self, shader_handle: &ShaderHandle) {
		self.shader = shader_handle.get_id();
	}

	pub fn add_index_buffer<T>(&mut self, index_buffer_handle: &BufferHandle<T>) {
		self.indices = index_buffer_handle.get_id();
	}

	pub fn add_buffer<T>(&mut self, buffer_handle: &BufferHandle<T>, attrib_loc: GLuint) {
		// push only if the handle isn't already there
		if !self.buffers.iter().any(|&(h, _)| h == buffer_handle.get_id()) {
			self.buffers.push((buffer_handle.get_id(), attrib_loc));
		}
	}

	pub fn remove_buffer<T>(&mut self, buffer_handle: &BufferHandle<T>) {
		// find the index (or return)
		let index = match self.buffers.iter().position(|&(h, _)| h == buffer_handle.get_id()) {
			Some(index) => index,
			None => return,
		};
		
		// remove at said index
		self.buffers.swap_remove(index);
	}

	pub fn add_texture(&mut self, texture_handle: &TextureHandle, attrib_loc: GLint) {
		// push only if the handle isn't already there
		if !self.textures.iter().any(|&(h, _)| h == texture_handle.get_id()) {
			self.textures.push((texture_handle.get_id(), attrib_loc));
		}
	}

	pub fn remove_texture(&mut self, texture_handle: &TextureHandle) {
		// find the index (or return)
		let index = match self.textures.iter().position(|&(h, _)| h == texture_handle.get_id()) {
			Some(index) => index,
			None => return,
		};
		
		// remove at said index
		self.textures.swap_remove(index);
	}

	pub fn set_uniform(&mut self, loc: GLint, data: GLSLAny) {
		match self.uniforms.iter().position(|&(_, l)| l == loc) {
			Some(index) => {
				if match data {
					GLSLAny::None => true,
					_ => false,
				} {
					self.uniforms.swap_remove(index);
				} else {
					self.uniforms[index].0 = data;
				}
			}
			None => {
				self.uniforms.push((data, loc));
			}
		}
	}

	pub fn enable_depth(&mut self, arg1: GLenum) {
		self.depth = EnableDepth::Yes(arg1);
	}

	pub fn disable_depth(&mut self) {
		self.depth = EnableDepth::No;
	}

	pub fn enable_blend(&mut self, arg1: GLenum, arg2: GLenum) {
		self.blend = EnableBlend::Yes(arg1, arg2);
	}

	pub fn disable_blend(&mut self) {
		self.blend = EnableBlend::No;
	}
}