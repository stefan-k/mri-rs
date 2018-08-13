// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Spatial Dimensions

use std;

/// spatial dimensions
#[derive(Debug, Clone)]
pub enum SpatialDims<T> {
    /// One dimension
    OneD(T),
    /// Two dimensions
    TwoD(T, T),
    /// Three dimensions
    ThreeD(T, T, T),
}

impl SpatialDims<f64> {
    /// invert the values
    pub fn invert(&self) -> Self {
        match *self {
            SpatialDims::OneD(x) => SpatialDims::OneD(1.0 / x),
            SpatialDims::TwoD(x, y) => SpatialDims::TwoD(1.0 / x, 1.0 / y),
            SpatialDims::ThreeD(x, y, z) => SpatialDims::ThreeD(1.0 / x, 1.0 / y, 1.0 / z),
        }
    }
}

impl<T> SpatialDims<T>
where
    T: std::ops::Mul<Output = T> + std::clone::Clone,
{
    /// compute the product
    pub fn product(&self) -> T {
        // wtf
        match *self {
            SpatialDims::OneD(ref x) => x.clone(),
            SpatialDims::TwoD(ref x, ref y) => x.clone() * y.clone(),
            SpatialDims::ThreeD(ref x, ref y, ref z) => x.clone() * y.clone() * z.clone(),
        }
    }
}

/// Iterator thingy
pub struct SpatialDimsIntoIterator<T> {
    dims: SpatialDims<T>,
    index: usize,
}

impl<T: std::clone::Clone> IntoIterator for SpatialDims<T> {
    type Item = T;
    type IntoIter = SpatialDimsIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SpatialDimsIntoIterator {
            dims: self,
            index: 0,
        }
    }
}

impl<T: std::clone::Clone> Iterator for SpatialDimsIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let idx = self.index;
        let dims = self.dims.clone();
        self.index += 1;
        if idx == 0 {
            match dims {
                SpatialDims::OneD(x) => Some(x),
                SpatialDims::TwoD(x, _) => Some(x),
                SpatialDims::ThreeD(x, _, _) => Some(x),
            }
        } else if idx == 1 {
            match dims {
                SpatialDims::TwoD(_, y) => Some(y),
                SpatialDims::ThreeD(_, y, _) => Some(y),
                _ => None,
            }
        } else if idx == 2 {
            match dims {
                SpatialDims::ThreeD(_, _, z) => Some(z),
                _ => None,
            }
        } else {
            None
        }
    }
}
