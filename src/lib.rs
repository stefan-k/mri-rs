// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! MRI

enum KSpaceSample {
    KS1D(f64),
    KS2D(f64, f64),
    KS3D(f64, f64, f64),
}

struct EncodingField {
    field: Vec<f64>,
}

struct RFSensitivity {
    sens: Vec<(f64, f64)>,
}

pub struct EncodingMatrix {
    k: Vec<KSpaceSample>,
    psi: Vec<EncodingField>,
    rf: Vec<RFSensitivity>,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
