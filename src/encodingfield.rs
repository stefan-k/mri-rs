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
    /// field of view
    pub fov: SpatialDims<f64>,
}

impl EncodingField {
    pub fn linear_x(fov: SpatialDims<f64>, dimensions: SpatialDims<usize>) -> Self {
        let mut field: Vec<f64>;
        match (&dimensions, &fov) {
            (&SpatialDims::OneD(nx), &SpatialDims::OneD(fov_x)) => {
                field = Vec::with_capacity(nx);
                let step = 1.0 - 1.0 / (nx as f64);
                for x in 0..nx {
                    field.push(fov_x * (-0.5 + (x as f64) * step));
                }
            }
            (&SpatialDims::TwoD(nx, ny), &SpatialDims::TwoD(fov_x, _)) => {
                field = Vec::with_capacity(nx * ny);
                let step = 1.0 - 1.0 / (nx as f64);
                for _ in 0..ny {
                    for x in 0..nx {
                        field.push(fov_x * (-0.5 + (x as f64) * step));
                    }
                }
            }
            (&SpatialDims::ThreeD(nx, ny, nz), &SpatialDims::ThreeD(fov_x, _, _)) => {
                field = Vec::with_capacity(nx * ny * nz);
                let step = 1.0 - 1.0 / (nx as f64);
                for _ in 0..nz {
                    for _ in 0..ny {
                        for x in 0..nx {
                            field.push(fov_x * (-0.5 + (x as f64) * step));
                        }
                    }
                }
            }
            _ => panic!("wrong"),
        }
        EncodingField {
            field,
            dimensions,
            fov,
        }
    }

    pub fn linear_y(fov: SpatialDims<f64>, dimensions: SpatialDims<usize>) -> Self {
        let mut field: Vec<f64>;
        match (&dimensions, &fov) {
            (&SpatialDims::OneD(_), &SpatialDims::OneD(_)) => {
                panic!("No y gradient in 1D problems");
            }
            (&SpatialDims::TwoD(nx, ny), &SpatialDims::TwoD(_, fov_y)) => {
                field = Vec::with_capacity(nx * ny);
                let step = 1.0 - 1.0 / (ny as f64);
                for y in 0..ny {
                    for _ in 0..nx {
                        field.push(fov_y * (-0.5 + (y as f64) * step));
                    }
                }
            }
            (&SpatialDims::ThreeD(nx, ny, nz), &SpatialDims::ThreeD(_, fov_y, _)) => {
                field = Vec::with_capacity(nx * ny * nz);
                let step = 1.0 - 1.0 / (ny as f64);
                for _ in 0..nz {
                    for y in 0..ny {
                        for _ in 0..nx {
                            field.push(fov_y * (-0.5 + (y as f64) * step));
                        }
                    }
                }
            }
            _ => panic!("wrong"),
        }
        EncodingField {
            field,
            dimensions,
            fov,
        }
    }
}
