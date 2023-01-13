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

#[derive(Copy, Clone, Debug)]
struct PerAmogusVertexData {
    model_matrix: [[f32; 4]; 4],
    color: [f32; 3],
    animation_frame: u8,
}
glium::implement_vertex!(PerAmogusVertexData, model_matrix, color, animation_frame);

pub struct AmogusData {
    pub world_position: nalgebra::Vector2<f32>,
    pub size: f32,
    pub color: [f32; 3],
    pub animation_frame: u8,
}

pub struct AmogusRenderer {
    vertex_buffer: glium::VertexBuffer<VertexData>,
    index_buffer: glium::IndexBuffer<u8>,
    shader: glium::Program,
    amogus_animation_texture: glium::texture::Texture2dArray,
    amogus_animation_dyeing_mask_texture: glium::texture::Texture2dArray,
    pub frames_number: u8,
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
        let frames_number = amogus_animation_texture.array_size() as u8;
        Self {
            vertex_buffer,
            index_buffer,
            shader,
            amogus_animation_texture,
            amogus_animation_dyeing_mask_texture,
            frames_number,
        }
    }

    pub fn draw(
        &self,
        display: &glium::Display,
        frame: &mut glium::Frame,
        amogsuses_data: Vec<AmogusData>,
    ) {
        let projection_matrix = nalgebra::Orthographic3::new(
            0.0,
            frame.get_dimensions().0 as f32,
            0.0,
            frame.get_dimensions().1 as f32,
            -1.0,
            1.0,
        );
        let projection_matrix_array: [[f32; 4]; 4] = *projection_matrix.as_matrix().as_ref();

        let amogus_instances: Vec<PerAmogusVertexData> = amogsuses_data
            .iter()
            .map(|single_amogus_data| {
                let mut model_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::identity();
                model_matrix.append_scaling_mut(single_amogus_data.size / 2.0);
                model_matrix.append_translation_mut(&nalgebra::Vector3::new(
                    single_amogus_data.world_position.x,
                    single_amogus_data.world_position.y,
                    0.0,
                ));

                PerAmogusVertexData {
                    model_matrix: *model_matrix.as_ref(),
                    color: single_amogus_data.color,
                    animation_frame: single_amogus_data.animation_frame,
                }
            })
            .collect();
        // println!("{:?}", amogus_instances[0].model_matrix);
        let instance_buffer =
            glium::vertex::VertexBuffer::new(display, amogus_instances.as_slice()).unwrap();
        frame
            .draw(
                (&self.vertex_buffer, instance_buffer.per_instance().unwrap()),
                &self.index_buffer,
                &self.shader,
                &glium::uniform! {
                    projection: projection_matrix_array,
                    tex_amogus: self.amogus_animation_texture.sampled(),
                    tex_amogus_dyeing_mask: self.amogus_animation_dyeing_mask_texture.sampled(),
                },
                &glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
    }
}
