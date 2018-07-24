// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Encoding fields

/// spatial dimensions
pub enum SpatialDims<T> {
    OneD(T),
    TwoD(T, T),
    ThreeD(T, T, T),
}

/// todo
pub struct EncodingField {
    /// actual field
    pub field: Vec<f64>,
    /// dimensions
    pub dimensions: SpatialDims<usize>,
}

impl EncodingField {
    pub fn linear_x(_fov: SpatialDims<f64>, dimensions: SpatialDims<usize>) -> Self {
        let mut field: Vec<f64>;
        match dimensions {
            SpatialDims::OneD(nx) => {
                field = Vec::with_capacity(nx);
                for x in 0..nx {
                    // todo
                    field.push(x as f64);
                }
            }
            SpatialDims::TwoD(nx, ny) => {
                field = Vec::with_capacity(nx * ny);
            }
            SpatialDims::ThreeD(nx, ny, nz) => {
                field = Vec::with_capacity(nx * ny * nz);
            }
        }
        EncodingField { field, dimensions }
    }

    pub fn linear_y(_fov: Vec<f64>, dimensions: SpatialDims<usize>) -> Self {
        let mut field: Vec<f64>;
        match dimensions {
            SpatialDims::OneD(nx) => {
                field = Vec::with_capacity(nx);
            }
            SpatialDims::TwoD(nx, ny) => {
                field = Vec::with_capacity(nx * ny);
            }
            SpatialDims::ThreeD(nx, ny, nz) => {
                field = Vec::with_capacity(nx * ny * nz);
            }
        }
        EncodingField { field, dimensions }
    }
}
