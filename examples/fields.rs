// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate mri;

use mri::encodingfield::EncodingFieldDiscrete;
use mri::SpatialDims;

fn main() {
    let fov = SpatialDims::OneD(1.0);
    let dims = SpatialDims::OneD(8);
    // let fov = SpatialDims::TwoD(1.0, 1.0);
    // let dims = SpatialDims::TwoD(8, 8);
    let f = EncodingFieldDiscrete::linear_x(fov, dims);
    println!("{:?}", f);
}
