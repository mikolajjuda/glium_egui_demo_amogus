use glium::Surface;
use glium::{self};
use image::GenericImage;

const TEXTURE_AMOGUS: &[u8] = include_bytes!("../resources/amogus.png");
const TEXTURE_AMOGUS_DYEING_MASK: &[u8] = include_bytes!("../resources/amogus_dyeing_mask.png");

fn texture2darray_from_png_bytes(
    image_bytes: &[u8],
    display: &glium::Display,
) -> glium::texture::Texture2dArray {
    let mut img =
        image::load_from_memory_with_format(image_bytes, image::ImageFormat::Png).unwrap();
    let animation_frame_side_size = img.height(); //animation frames are squares
    let animation_frames_num = img.width() / animation_frame_side_size;
    let animation_frames = (0..animation_frames_num)
        .map(|frame_id| {
            let frame = img
                .sub_image(
                    frame_id * animation_frame_side_size,
                    0,
                    animation_frame_side_size,
                    animation_frame_side_size,
                )
                .to_image();
            let frame_size = (frame.width(), frame.height());
            glium::texture::RawImage2d::from_raw_rgba(frame.into_raw(), frame_size)
        })
        .collect();
    glium::texture::Texture2dArray::new(display, animation_frames).unwrap()
}

#[derive(Copy, Clone)]
struct VertexData {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
glium::implement_vertex!(VertexData, position, tex_coords);

const VERTEX_DATA: &[VertexData] = &[
    VertexData {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 0.0],
    },
    VertexData {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 1.0],
    },
    VertexData {
        position: [1.0, -1.0],
        tex_coords: [1.0, 1.0],
    },
    VertexData {
        position: [1.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
];

const INDEX_DATA: &[u8] = &[0, 1, 3, 3, 1, 2];

#[derive(Copy, Clone)]
pub struct AmogusData {
    world_position: [f32; 2],
    color: [f32; 3],
    animation_frame: u8,
}
glium::implement_vertex!(AmogusData, world_position, color, animation_frame);

pub struct AmogusRenderer {
    vertex_buffer: glium::VertexBuffer<VertexData>,
    index_buffer: glium::IndexBuffer<u8>,
    shader: glium::Program,
    amogus_animation_texture: glium::texture::Texture2dArray,
    amogus_animation_dyeing_mask_texture: glium::texture::Texture2dArray,
}

impl AmogusRenderer {
    pub fn new(display: &glium::Display) -> Self {
        let vertex_buffer = glium::vertex::VertexBuffer::new(display, VERTEX_DATA).unwrap();
        let index_buffer = glium::index::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            INDEX_DATA,
        )
        .unwrap();
        let shader = glium::program::Program::from_source(
            display,
            include_str!("amogus_shader_vertex.glsl"),
            include_str!("amogus_shader_fragment.glsl"),
            None,
        )
        .unwrap();
        let amogus_animation_texture = texture2darray_from_png_bytes(TEXTURE_AMOGUS, display);
        let amogus_animation_dyeing_mask_texture =
            texture2darray_from_png_bytes(TEXTURE_AMOGUS_DYEING_MASK, display);
        Self {
            vertex_buffer,
            index_buffer,
            shader,
            amogus_animation_texture,
            amogus_animation_dyeing_mask_texture,
        }
    }

    pub fn draw(&self, frame: &mut glium::Frame) {
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.shader,
                &glium::uniform! {
                    tex_amogus: self.amogus_animation_texture.sampled(),
                    tex_amogus_dyeing_mask: self.amogus_animation_dyeing_mask_texture.sampled(),
                    anim_frame: 0u8
                },
                &glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
    }
}
