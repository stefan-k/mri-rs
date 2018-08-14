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
}

impl<'a> LocalKSpace<'a> {
    /// Create new local k-space object
    pub fn new(kspace: KSpace, fields: Vec<EncodingField<'a>>) -> Self {
        assert!(kspace.num_channels() == fields.len());
        LocalKSpace {
            kspace: kspace,
            fields: fields,
        }
    }

    /// return local k space a certain position
    pub fn at(&self, pos: SpatialDims<f64>) -> KSpace {
        let derivs: Vec<SpatialDims<f64>> = self.fields.iter().map(|x| x.deriv_at(&pos)).collect();
        let mut grad: Vec<Vec<f64>> = Vec::with_capacity(pos.len());
        grad.push(derivs.iter().map(|x| x.x().unwrap()).collect());
        if pos.len() > 1 {
            grad.push(derivs.iter().map(|x| x.y().unwrap()).collect());
        }
        if pos.len() > 2 {
            grad.push(derivs.iter().map(|x| x.z().unwrap()).collect());
        }

        let mut out = KSpace::new();
        out.add_samples(
            self.kspace
                .kspace
                .iter()
                .map(|x| {
                    grad.iter()
                        .map(|c| x.iter().zip(c.iter()).map(|(a, b)| a * b).sum())
                        .collect()
                }).collect(),
        );
        out
    }
}
