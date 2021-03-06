#[macro_use]
extern crate glium;
extern crate image;

/**
    Tutorial obtenido de https://github.com/glium/glium/tree/master/book
**/

fn main() {
    println!("Welcome to opengl glium");

    use glium::{glutin,Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window,context,&events_loop).unwrap();

    use std::io::Cursor;

    //TODO : Incluye una ruta a una imagen &include_bytes ...

    let image = image::load(Cursor::new(&include_bytes!("")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex{
        position: [f32; 2],
        tex_cords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_cords);

    let vertex1 = Vertex {position: [-0.5,-0.5], tex_cords: [0.0, 0.0]};
    let vertex2 = Vertex {position: [0.0,0.5], tex_cords: [0.0, 1.0]};
    let vertex3 = Vertex {position: [0.5,-0.25], tex_cords: [1.0, 0.0]};
    let shape = vec![vertex1,vertex2,vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main(){
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position,0.0,1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        void main(){
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut closed = false;
    let mut t:f32 = -0.5;
    while !closed{

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0,0.0,1.0,1.0);

        let uniforms = uniform!{
            matrix: [
                [t.cos() , t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
                tex: &texture,
            ]
        };

        target.draw(&vertex_buffer,&indices,&program,&uniforms,&Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev|{
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
