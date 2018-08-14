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

    let sema = &|pos: &SpatialDims<f64>| pos.x().unwrap().powi(2) - pos.y().unwrap().powi(2);
    let dsema = &|pos: &SpatialDims<f64>| {
        SpatialDims::TwoD(2.0 * pos.x().unwrap(), -2.0 * pos.y().unwrap())
    };
    let mut fa = EncodingField::new(&sema);
    fa.derivative(&dsema);

    let semb = &|pos: &SpatialDims<f64>| 2.0 * pos.x().unwrap() * pos.y().unwrap();
    let dsemb =
        &|pos: &SpatialDims<f64>| SpatialDims::TwoD(2.0 * pos.y().unwrap(), 2.0 * pos.x().unwrap());
    let mut fb = EncodingField::new(&semb);
    fb.derivative(&dsemb);

    let ks = KSpace::cartesian(SpatialDims::TwoD(fov, fov), SpatialDims::TwoD(nx, nx));
    println!("{:?}", ks);

    let lk = LocalKSpace::new(ks, vec![fa, fb]);
    let localk = lk.at(&SpatialDims::TwoD(0.1, 0.1));

    println!("{:?}", localk);
}
