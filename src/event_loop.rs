use std::{convert::TryInto, fs};

use glium::{Display, Surface, glutin::{self, event_loop::EventLoop}, uniform};

use crate::{Vertex, graphing::graph_data, renderer::Renderer, traits::VectorUnnormalizedValues};

pub fn event_loop_function(event_loop: EventLoop<()>, display: Display) {
    let vertex_shader = fs::read_to_string("shader.vert").expect("Something went wrong when reading the file");
    let fragment_shader = fs::read_to_string("shader.frag").expect("Something went wrong when reading the file");

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    // Vertices of an object
    let vertex1 = Vertex { position: [-300.0, -300.0] };
    let vertex2 = Vertex { position: [ 300.0,  300.0] };
    let vertex3 = Vertex { position: [ 300.0, -300.0] };
    let triangle = vec![vertex1, vertex2, vertex3];

    // Renderer struct to make it easy to manage static and dynamic objects
    let mut renderer = Renderer::new();
    renderer.add_to_dynamic_from_vec(&triangle);

    // Size of the screen
    let mut size = display.gl_window().window().inner_size().to_logical::<f32>(1.0);

    // Vector of Vertices to use to draw stuff
    let mut shape_vertices = renderer.constant.clone();
    shape_vertices.append(&mut renderer.dynamic);
    shape_vertices = shape_vertices.unnormalize_values(size.clone());

    // Drawing Stuff
    let mut vertex_buffer = glium::VertexBuffer::new(&display, &shape_vertices).unwrap();
    let mut program = glium::Program::from_source(&display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();

    let mut t: f32 = 0.0;
    let mut data: Vec<i32> = vec![];

    event_loop.run(move |ev, _, control_flow| {
        let last = std::time::Instant::now();
        
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    graph_data(data.clone()).unwrap();
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::RedrawRequested(_) => {
                size = display.gl_window().window().inner_size().to_logical::<f32>(1.0);
                renderer.update();
                renderer.add_to_dynamic_from_vec(&triangle);

                // Get the Vec<Vertex> Again
                shape_vertices = renderer.constant.clone();
                shape_vertices.append(&mut renderer.dynamic);
                shape_vertices = shape_vertices.unnormalize_values(size.clone());

                vertex_buffer = glium::VertexBuffer::new(&display, &shape_vertices).unwrap();
                program = glium::Program::from_source(&display, vertex_shader.as_str(), 
                fragment_shader.as_str(), None).unwrap();
            },
            _ => (),
        }

        t += 0.01/size.width;

        let mut target = display.draw();
        target.clear_color(255.0/255.0, 180.0/255.0, 180.0/255.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t }, &Default::default()).unwrap();
        target.finish().unwrap();
        data.push(last.elapsed().as_nanos() as i32)
    });
}
