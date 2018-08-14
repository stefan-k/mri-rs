// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate mri;

use mri::EncodingField;
use mri::KSpace;
use mri::LocalKSpace;
use mri::SpatialDims;

fn main() {
    // in mm
    let fov: f64 = 0.2;
    let nx: usize = 4;

    let linx = &|pos: &SpatialDims<f64>| pos.x().unwrap();
    let dlinx = &|_pos: &SpatialDims<f64>| SpatialDims::TwoD(1.0, 0.0);
    let mut fx = EncodingField::new(&linx);
    fx.derivative(&dlinx);

    let liny = &|pos: &SpatialDims<f64>| pos.y().unwrap();
    let dliny = &|_pos: &SpatialDims<f64>| SpatialDims::TwoD(0.0, 1.0);
    let mut fy = EncodingField::new(&liny);
    fy.derivative(&dliny);

    let ks = KSpace::cartesian(SpatialDims::TwoD(fov, fov), SpatialDims::TwoD(nx, nx));
    println!("{:?}", ks);

    let lk = LocalKSpace::new(ks, vec![fx, fy]);
    let localk = lk.at(SpatialDims::TwoD(0.1, -0.1));

    println!("{:?}", localk);
}
