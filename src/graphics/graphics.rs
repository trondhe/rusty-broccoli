#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;
extern crate gfx_hal as hal;

use std::sync::Arc;
use winit::Window;

struct Vertex {
    a_pos: [f32; 3],
    a_uv: [f32; 2],
}

pub struct SwapchainConfig;

pub struct Graphics;

pub fn gfxloop() {}
