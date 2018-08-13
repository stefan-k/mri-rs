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
    // in mm
    let fov: f64 = 0.2;
    let nx: usize = 4;

    let linx = &|x: f64, _: f64, _: f64| x;
    let dlinx = &|_: f64, _: f64, _: f64| (1.0, 0.0, 0.0);
    let mut fx = EncodingField::new(&linx);
    fx.derivative(&dlinx);

    let liny = &|_: f64, y: f64, _: f64| y;
    let dliny = &|_: f64, _: f64, _: f64| (0.0, 1.0, 0.0);
    let mut fy = EncodingField::new(&liny);
    fy.derivative(&dliny);

    println!("{:?}", fx.at(2.0, 0.0, 0.0));

    let ks = KSpace::cartesian(SpatialDims::TwoD(fov, fov), SpatialDims::TwoD(nx, nx));
    println!("{:?}", ks);

    let pos_x = 0.1;
    let pos_y = -0.1;
    let pos_z = 0.0;

    let diff_x = fx.deriv_at(pos_x, pos_y, pos_z);
    let diff_y = fy.deriv_at(pos_x, pos_y, pos_z);

    println!("{:?} | {:?}", diff_x, diff_y);
}
