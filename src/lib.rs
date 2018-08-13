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
use kspace::KSpace;
use rf::RFSensitivity;

/// spatial dimensions
#[derive(Debug)]
pub enum SpatialDims<T> {
    /// One dimension
    OneD(T),
    /// Two dimensions
    TwoD(T, T),
    /// Three dimensions
    ThreeD(T, T, T),
}

/// todo
pub struct EncodingMatrix {
    /// kspace
    k: KSpace,
    /// vector of encoding fields
    psi: Vec<EncodingField>,
    //// rf sensitivity maps
    rf: Vec<RFSensitivity>,
}

impl EncodingMatrix {
    /// todo
    pub fn new(k: KSpace, psi: Vec<EncodingField>, rf: Vec<RFSensitivity>) -> Self {
        EncodingMatrix { k, psi, rf }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
