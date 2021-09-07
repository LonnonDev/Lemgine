use crate::Vertex;

#[derive(Clone)]
pub struct Renderer {
    pub constant: Vec<Vertex>,
    pub dynamic: Vec<Vertex>,
}

#[allow(dead_code)]
impl Renderer {
    pub fn new() -> Renderer {
        return Renderer { 
            constant: vec![], 
            dynamic: vec![],
        }
    }
    pub fn add_to_constant(&mut self, vertex: &Vertex) {
        self.constant.push(*vertex);
    }
    pub fn add_to_constant_from_vec(&mut self, vertex: &Vec<Vertex>) {
        for x in vertex {
            self.constant.push(*x);
        }
    }
    pub fn add_to_dynamic(&mut self, vertex: &Vertex) {
        self.dynamic.push(*vertex);
    }
    pub fn add_to_dynamic_from_vec(&mut self, vertex: &Vec<Vertex>) {
        for x in vertex {
            self.dynamic.push(*x);
        }
    }
    pub fn update(&mut self) {
        self.dynamic = vec![];
    }
}