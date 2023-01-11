use glium;

const TEXTURE_AMOGUS: &[u8] = include_bytes!("../resources/amogus.png");
const TEXTURE_AMOGUS_DYEING_MASK: &[u8] = include_bytes!("../resources/amogus_dyeing_mask.png");

#[derive(Copy, Clone)]
struct VertexData {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
glium::implement_vertex!(VertexData, position, tex_coords);

#[derive(Copy, Clone)]
pub struct AmogusData {
    world_position: [f32; 2],
    color: [f32; 3],
    animation_frame: u8
}
glium::implement_vertex!(AmogusData, world_position, color, animation_frame);

pub struct AmogusRenderer {
    vertex_buffer: glium::VertexBuffer<VertexData>,
    index_buffer: glium::IndexBuffer<u8>,
    shader: glium::Program,
    amogus_animation_grayscale_texture: glium::texture::Texture2dArray,
    amogus_animation_not_visor_mask_texture: glium::texture::Texture2dArray,
}
