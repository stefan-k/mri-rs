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

// fn localkspace2(grad_x: Vec<f64>, grad_y: Vec<f64>, grad_z: Vec<f64>, k: &KSpace) -> KSpace {
fn localkspace2(grad_x: Vec<f64>, grad_y: Vec<f64>, k: &KSpace) -> KSpace {
    assert!(grad_x.len() == grad_y.len());
    // assert!(grad_y.len() == grad_z.len());
    let mut out = KSpace::new();
    out.add_samples(
        k.kspace
            .iter()
            .map(|x| {
                vec![
                    x.iter().zip(grad_x.iter()).map(|(a, b)| a * b).sum(),
                    x.iter().zip(grad_y.iter()).map(|(a, b)| a * b).sum(),
                    // x.iter().zip(grad_z.iter()).map(|(a, b)| a * b).sum(),
                ]
            }).collect(),
    );
    out
}

fn main() {
    // in mm
    let fov: f64 = 0.2;
    let nx: usize = 4;

    let linx = &|pos: &SpatialDims<f64>| pos.x().unwrap();
    let dlinx = &|pos: &SpatialDims<f64>| SpatialDims::TwoD(1.0, 0.0);
    let mut fx = EncodingField::new(&linx);
    fx.derivative(&dlinx);

    let liny = &|pos: &SpatialDims<f64>| pos.y().unwrap();
    let dliny = &|pos: &SpatialDims<f64>| SpatialDims::TwoD(0.0, 1.0);
    let mut fy = EncodingField::new(&liny);
    fy.derivative(&dliny);

    println!("{:?}", fx.at(&SpatialDims::TwoD(2.0, 0.0)));

    let ks = KSpace::cartesian(SpatialDims::TwoD(fov, fov), SpatialDims::TwoD(nx, nx));
    println!("{:?}", ks);

    let pos_x = 0.1;
    let pos_y = -0.1;
    let pos_z = 0.0;

    let diff_fx = fx.deriv_at(&SpatialDims::ThreeD(pos_x, pos_y, pos_z));
    let diff_fy = fy.deriv_at(&SpatialDims::ThreeD(pos_x, pos_y, pos_z));

    println!("{:?} | {:?}", diff_fx, diff_fy);

    let grad_x = vec![diff_fx.x().unwrap(), diff_fy.x().unwrap(), 0.0];
    let grad_y = vec![diff_fx.y().unwrap(), diff_fy.y().unwrap(), 0.0];
    // let grad_z = vec![diff_fx.z().unwrap(), diff_fy.z().unwrap(), 0.0];

    let localk = localkspace2(grad_x, grad_y, &ks);
    println!("{:?}", localk);

    let lk = LocalKSpace::new(ks, vec![fx, fy]);
    let lokalk = lk.at(SpatialDims::TwoD(0.0, 1.0));

    println!("{:?}", localk);
}
