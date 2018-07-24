// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! k-Space

type KSample = Vec<f64>;

pub struct KSpace {
    kspace: Vec<KSample>,
    num_channels: usize,
}

impl KSpace {
    pub fn new() -> Self {
        KSpace {
            kspace: vec![],
            num_channels: 0,
        }
    }

    pub fn add_sample(&mut self, sample: KSample) -> &mut Self {
        if self.num_channels == 0 {
            self.num_channels = sample.len();
        } else if self.num_channels != sample.len() {
            panic!("Wrong number of samples");
        }

        self.kspace.push(sample);
        self
    }

    pub fn add_samples(&mut self, samples: Vec<KSample>) -> &mut Self {
        samples
            .into_iter()
            .map(|sample| {
                self.add_sample(sample);
            })
            .count();
        self
    }
}
