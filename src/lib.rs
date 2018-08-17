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

extern crate num;

pub mod encodingfield;
pub mod kspace;
pub mod localkspace;
pub mod rf;
pub mod spatialdims;

pub use encodingfield::EncodingField;
pub use kspace::KSample;
pub use kspace::KSpace;
pub use kspace::KSpaceThings;
pub use localkspace::LocalKSpace;
pub use rf::RFSensitivity;
pub use spatialdims::SpatialDims;

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
