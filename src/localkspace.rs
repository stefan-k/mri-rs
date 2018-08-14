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
        // let mut grads: Vec<Vec<f64>> = Vec::with_capacity(num_channels);
        LocalKSpace {
            kspace: kspace,
            fields: fields,
            num_channels: num_channels,
        }
    }

    /// return local k space a certain position
    pub fn at(&self, pos: SpatialDims<f64>) -> KSpace {
        // let grads: Vec<Vec<f64>> = Vec::with_capacity(self.num_channels);
        let grad_x = self.fields[0].deriv_at(pos);
        unimplemented!()
    }
}
