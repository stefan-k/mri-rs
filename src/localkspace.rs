// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Local k-Space

use EncodingField;
use KSpace;
use SpatialDims;

/// Local k-Space
pub struct LocalKSpace<'a> {
    /// Actual k-Space
    kspace: KSpace,
    /// fields
    fields: Vec<EncodingField<'a>>,
    /// number of channels
    num_channels: usize,
}

impl<'a> LocalKSpace<'a> {
    /// Create new local k-space object
    pub fn new(kspace: KSpace, fields: Vec<EncodingField<'a>>) -> Self {
        assert!(kspace.num_channels() == fields.len());
        let num_channels = fields.len();
        LocalKSpace {
            kspace: kspace,
            fields: fields,
            num_channels: num_channels,
        }
    }

    /// return local k space a certain position
    pub fn at(&self, pos: SpatialDims<f64>) -> KSpace {
        let derivs: Vec<SpatialDims<f64>> = self.fields.iter().map(|x| x.deriv_at(&pos)).collect();
        let grad_x: Vec<f64> = derivs.iter().map(|x| x.x().unwrap()).collect();
        let grad_y: Vec<f64> = derivs.iter().map(|x| x.y().unwrap()).collect();
        let mut out = KSpace::new();
        out.add_samples(
            self.kspace
                .kspace
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
}
