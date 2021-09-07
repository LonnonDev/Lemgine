use std::convert::TryInto;

use glium::glutin::dpi::LogicalSize;

use crate::Vertex;


#[derive(PartialEq, Clone, Copy)]
pub enum WindowDivide {
    Height,
    Width,
}

pub trait ConvertToUnnormalized where Self: Sized {
    fn to_unnormalized(&mut self, window_size: LogicalSize<f32>, divide_type: WindowDivide) -> Self;
}

impl ConvertToUnnormalized for f32 {
    fn to_unnormalized(&mut self, window_size: LogicalSize<f32>, divide_type: WindowDivide) -> Self {
        let mut divide: f32 = 0.0;
        if divide_type == WindowDivide::Width {
            divide = window_size.width;
        } else if divide_type == WindowDivide::Height {
            divide = window_size.height;
        }
        
        let return_value = *self/divide;
        return return_value
    }
}

pub trait VectorUnnormalizedValues {
    fn unnormalize_values(&mut self, window_size: LogicalSize<f32>) -> Vec<Vertex>;
}

impl VectorUnnormalizedValues for Vec<Vertex> {
    fn unnormalize_values(&mut self, window_size: LogicalSize<f32>) -> Self {
        let mut new_vector = vec![];
        for vertex in self.clone() {
            let mut new_position = Vertex { position: [0.0, 0.0] };
            let mut position = vec![];
            let mut index = 0;
            for mut x in vertex.position {
                let divide_type = match index {
                    0 => WindowDivide::Width,
                    1 => WindowDivide::Height,
                    _ => panic!("Index Overflowed"),
                };
                position.push(x.to_unnormalized(window_size.clone(), divide_type));
                index += 1;
            }
            new_position.position = demo(position);
            new_vector.push(new_position);
        }
        
        return new_vector
    }
}

fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}