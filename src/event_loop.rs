use std::fs;

use glium::{Display, Surface, glutin::{self, event_loop::EventLoop}, uniform};

use crate::{Vertex, renderer::Renderer};

pub fn event_loop_function(event_loop: EventLoop<()>, display: Display, mut shape: Renderer) {
    let vertex_shader = fs::read_to_string("shader.vert").expect("Something went wrong when reading the file");
    let fragment_shader = fs::read_to_string("shader.frag").expect("Something went wrong when reading the file");

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    

    let mut t: f32 = 0.0;

    event_loop.run(move |ev, _, control_flow| {
        shape.update();
        let mut shape_vertices = shape.constant.clone();
        shape_vertices.append(&mut shape.dynamic);
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape_vertices).unwrap();
        let program = glium::Program::from_source(&display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
        
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
        // t += 0.01/255.0;

        let mut target = display.draw();
        target.clear_color(255.0/255.0, 180.0/255.0, 180.0/255.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t }, &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
