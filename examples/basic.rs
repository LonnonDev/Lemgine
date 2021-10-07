use backend::{Surface, uniform};
use lemgine::{backend::{self, glutin::event::{DeviceId, KeyboardInput}}, engine::Variable, traits::VectorUnnormalizedValues, vertex::Vertex};
use lemgine::{engine::{WindowDrawer}, renderer::Renderer};

trait Rendering {
    fn render_cube(&mut self);
    fn renderer(&mut self);
}

impl Rendering for WindowDrawer {
    fn render_cube(&mut self) {
        // Get and print the x variable
        let x: f32 = self.get_var("x");
        println!("{}", x);

        // Vertices of an object
        let vertex1 = Vertex { position: [-100.0, -100.0] };
        let vertex2 = Vertex { position: [ 100.0,  100.0] };
        let vertex3 = Vertex { position: [ 100.0, -100.0] };
        let vertex4 = Vertex { position: [ 100.0,  100.0] };
        let vertex5 = Vertex { position: [-100.0, -100.0] };
        let vertex6 = Vertex { position: [-100.0,  100.0] };
        let vertices = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
        self.renderer.add_to_dynamic_from_vec(&vertices);

        // Make background color
        let mut target = self.display.draw();
        target.clear_color(255.0/255.0, 180.0/255.0, 180.0/255.0, 1.0);
        target.draw(&self.vertex_buffer, &self.indices, &self.program, &uniform!{ }, &Default::default()).unwrap();
        target.finish().unwrap();
    }
    fn renderer(&mut self) {
        // Add the vertices to the shape vertices Vec
        self.shape_vertices = self.renderer.constant.clone();
        self.shape_vertices.append(&mut self.renderer.dynamic);
        self.shape_vertices = self.shape_vertices.unnormalize_values(self.size.clone());

        // Make vertex buffer
        self.vertex_buffer = backend::VertexBuffer::new(&self.display, &self.shape_vertices).unwrap();
        // Make program from shaders
        self.program = backend::Program::from_source(&self.display, self.vertex_shader.as_str(), 
        self.fragment_shader.as_str(), None).unwrap();
    }
}

trait Input {
    fn get_movement(&mut self, device_id: DeviceId, input: KeyboardInput, is_synthetic: bool); 
}

impl Input for WindowDrawer {
    fn get_movement(&mut self, _: DeviceId, _: KeyboardInput, _: bool) {
        
    }
}

fn main() {
    // Make event loop
    let event_loop = backend::glutin::event_loop::EventLoop::new();
    let wb = backend::glutin::window::WindowBuilder::new()
        .with_inner_size(backend::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = backend::glutin::ContextBuilder::new();
    let display = backend::Display::new(wb.clone(), cb, &event_loop).unwrap();

    // Shaders
    let vertex = "
#version 140

in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}    
".to_string();
    let fragment = "
#version 140

out vec4 color;

void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
}
".to_string();

    // Make window drawer
    let mut window_drawer = WindowDrawer::new(vertex, fragment, display, Renderer::new());
    // Add a the variable "x"
    window_drawer.add_variable("x", Variable::F32(0f32));
    // if you put functions later in the vec it will be ran later
    window_drawer.run(event_loop, vec![Rendering::render_cube, Rendering::renderer], vec![Input::get_movement], vec![]);
}