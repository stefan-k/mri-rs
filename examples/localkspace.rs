// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate mri;

use mri::EncodingField;
use mri::KSpace;
use mri::SpatialDims;

fn main() {
    let linx = &|x: f64, _: f64, _: f64| x;
    let dlinx = &|_: f64, _: f64, _: f64| (1.0, 0.0, 0.0);
    let mut fx = EncodingField::new(&linx);
    fx.derivative(&dlinx);

    let liny = &|_: f64, y: f64, _: f64| y;
    let dliny = &|_: f64, _: f64, _: f64| (0.0, 1.0, 0.0);
    let mut fy = EncodingField::new(&liny);
    fy.derivative(&dliny);

    println!("{:?}", fx.at(2.0, 0.0, 0.0));

    let ks = KSpace::cartesian(SpatialDims::TwoD(2.0, 2.0), SpatialDims::TwoD(8, 8));
    println!("{:?}", ks);
}
