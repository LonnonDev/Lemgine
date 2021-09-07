use crate::Vertex;

#[derive(Clone)]
pub struct Renderer {
    pub constant: Vec<Vertex>,
    pub dynamic: Vec<Vertex>
}

impl Renderer {
    pub fn new() -> Renderer {
        return Renderer { 
            constant: vec![], 
            dynamic: vec![]
        }
    }
    pub fn add_to_constant(&mut self, vertex: Vertex) -> Self {
        self.constant.push(vertex);
        return self.clone()
    }
    pub fn add_to_constant_from_vec(&mut self, vertex: Vec<Vertex>) -> Self {
        for x in vertex {
            self.constant.push(x);
        }
        return self.clone()
    }
    pub fn add_to_dynamic(&mut self, vertex: Vertex) -> Self {
        self.dynamic.push(vertex);
        return self.clone()
    }
    pub fn add_to_dynamic_from_vec(&mut self, vertex: Vec<Vertex>) -> Self {
        for x in vertex {
            self.constant.push(x);
        }
        return self.clone()
    }
    pub fn update(&mut self) {
        self.dynamic = vec![];
    }
}