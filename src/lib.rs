// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! MRI

mod encodingfield;
mod kspace;
mod rf;

use encodingfield::EncodingField;
use kspace::KSpace;
use rf::RFSensitivity;

pub struct EncodingMatrix {
    k: KSpace,
    psi: Vec<EncodingField>,
    rf: Vec<RFSensitivity>,
}

impl EncodingMatrix {
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
