// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Encoding fields

use SpatialDims;

/// Different kinds of encoding field derivatives
enum EncodingFieldDerivative {
    FiniteDiff,
    Func(Box<Fn(&SpatialDims<f64>) -> SpatialDims<f64>>),
}

/// This is a field that will be computed on the fly
pub struct EncodingField {
    /// Field
    field: Box<Fn(&SpatialDims<f64>) -> f64>,
    /// derivative
    derivative: EncodingFieldDerivative,
}

impl EncodingField {
    /// Constructor
    pub fn new(field: Box<Fn(&SpatialDims<f64>) -> f64>) -> Self {
        EncodingField {
            field,
            derivative: EncodingFieldDerivative::FiniteDiff,
        }
    }

    /// Set derivative of the field
    pub fn derivative(
        &mut self,
        derivative: Box<Fn(&SpatialDims<f64>) -> SpatialDims<f64>>,
    ) -> &mut Self {
        self.derivative = EncodingFieldDerivative::Func(derivative);
        self
    }

    /// Get value of field at position (x, y, z)
    pub fn at(&self, pos: &SpatialDims<f64>) -> f64 {
        (*self.field)(pos)
    }

    /// Get the derivative at a certain point
    pub fn deriv_at(&self, pos: &SpatialDims<f64>) -> SpatialDims<f64> {
        match self.derivative {
            EncodingFieldDerivative::FiniteDiff => unimplemented!(),
            EncodingFieldDerivative::Func(ref f) => f(pos),
        }
    }
}

/// todo
#[derive(Debug)]
pub struct EncodingFieldDiscrete {
    /// actual field
    pub field: Vec<f64>,
    /// dimensions
    dimensions: SpatialDims<usize>,
    /// field of view
    fov: SpatialDims<f64>,
}

impl EncodingFieldDiscrete {
    /// Create a linear field in x
    pub fn linear_x(fov: SpatialDims<f64>, dimensions: SpatialDims<usize>) -> Self {
        let mut field: Vec<f64>;
        match (&dimensions, &fov) {
            (&SpatialDims::OneD(nx), &SpatialDims::OneD(fov_x)) => {
                field = Vec::with_capacity(nx);
                let step = 1.0 / (nx as f64);
                for x in 0..nx {
                    field.push(fov_x * (-0.5 + (x as f64) * step));
                }
            }
            (&SpatialDims::TwoD(nx, ny), &SpatialDims::TwoD(fov_x, _)) => {
                field = Vec::with_capacity(nx * ny);
                let step = 1.0 / (nx as f64);
                for _ in 0..ny {
                    for x in 0..nx {
                        field.push(fov_x * (-0.5 + (x as f64) * step));
                    }
                }
            }
            (&SpatialDims::ThreeD(nx, ny, nz), &SpatialDims::ThreeD(fov_x, _, _)) => {
                field = Vec::with_capacity(nx * ny * nz);
                let step = 1.0 / (nx as f64);
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

        EncodingFieldDiscrete {
            field,
            dimensions,
            fov,
        }
    }

    /// Create a linear field in y
    pub fn linear_y(fov: SpatialDims<f64>, dimensions: SpatialDims<usize>) -> Self {
        let mut field: Vec<f64>;
        match (&dimensions, &fov) {
            (&SpatialDims::OneD(_), &SpatialDims::OneD(_)) => {
                panic!("No y gradient in 1D problems");
            }
            (&SpatialDims::TwoD(nx, ny), &SpatialDims::TwoD(_, fov_y)) => {
                field = Vec::with_capacity(nx * ny);
                let step = 1.0 / (ny as f64);
                for y in 0..ny {
                    for _ in 0..nx {
                        field.push(fov_y * (-0.5 + (y as f64) * step));
                    }
                }
            }
            (&SpatialDims::ThreeD(nx, ny, nz), &SpatialDims::ThreeD(_, fov_y, _)) => {
                field = Vec::with_capacity(nx * ny * nz);
                let step = 1.0 / (ny as f64);
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

        EncodingFieldDiscrete {
            field,
            dimensions,
            fov,
        }
    }

    /// Create a linear field in z
    pub fn linear_z(fov: SpatialDims<f64>, dimensions: SpatialDims<usize>) -> Self {
        let mut field: Vec<f64>;
        match (&dimensions, &fov) {
            (&SpatialDims::OneD(_), &SpatialDims::OneD(_)) => {
                panic!("No z gradient in 1D problems");
            }
            (&SpatialDims::TwoD(_, _), &SpatialDims::TwoD(_, _)) => {
                panic!("No z gradient in 2D problems");
            }
            (&SpatialDims::ThreeD(nx, ny, nz), &SpatialDims::ThreeD(_, _, fov_z)) => {
                field = Vec::with_capacity(nx * ny * nz);
                let step = 1.0 / (ny as f64);
                for z in 0..nz {
                    for _ in 0..ny {
                        for _ in 0..nx {
                            field.push(fov_z * (-0.5 + (z as f64) * step));
                        }
                    }
                }
            }
            _ => panic!("wrong"),
        }

        EncodingFieldDiscrete {
            field,
            dimensions,
            fov,
        }
    }

    /// Return the dimensions
    pub fn dimensions(&self) -> SpatialDims<usize> {
        self.dimensions.clone()
    }

    /// Return the FOV
    pub fn fov(&self) -> SpatialDims<f64> {
        self.fov.clone()
    }
}
