// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! MRI

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(missing_docs)]

pub mod encodingfield;
pub mod kspace;
pub mod rf;

use encodingfield::EncodingField;
// use kspace::KSpace;
// use rf::RFSensitivity;

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

// /// todo
// pub struct EncodingMatrix {
//     /// kspace
//     k: KSpace,
//     /// vector of encoding fields
//     psi: Vec<EncodingField>,
//     //// rf sensitivity maps
//     rf: Vec<RFSensitivity>,
// }

// impl EncodingMatrix {
//     /// todo
//     pub fn new(k: KSpace, psi: Vec<EncodingField>, rf: Vec<RFSensitivity>) -> Self {
//         EncodingMatrix { k, psi, rf }
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
