// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! k-Space

use num::Integer;
use SpatialDims;
// use SpatialDimsIntoIterator;

type KSample = Vec<f64>;

/// Representation of a k-space trajectory
#[derive(Debug, Clone)]
pub struct KSpace {
    /// A vector of k-space samples
    pub kspace: Vec<KSample>,
    /// Number of encoding channels
    num_channels: usize,
}

impl KSpace {
    /// Constructor
    pub fn new() -> Self {
        KSpace {
            kspace: vec![],
            num_channels: 0,
        }
    }

    /// Add a single k-space sample point to an existing trajectory
    pub fn add_sample(&mut self, sample: KSample) -> &mut Self {
        if self.num_channels == 0 {
            self.num_channels = sample.len();
        } else if self.num_channels != sample.len() {
            panic!("Wrong number of samples");
        }

        self.kspace.push(sample);
        self
    }

    /// Add several k-space sample points to an existing trajectory
    pub fn add_samples(&mut self, samples: Vec<KSample>) -> &mut Self {
        samples
            .into_iter()
            .map(|sample| {
                self.add_sample(sample);
            }).count();
        self
    }

    /// Return the number of channels
    pub fn num_channels(&self) -> usize {
        self.num_channels
    }

    /// Return the number of k-space samples
    pub fn num_samples(&self) -> usize {
        self.kspace.len()
    }

    /// Create a Cartesian trajectory
    pub fn cartesian(fov: SpatialDims<f64>, samples: SpatialDims<usize>) -> Self {
        let dk = fov.invert();
        let mut k: Vec<KSample> = Vec::with_capacity(samples.product());
        match (dk, samples) {
            (SpatialDims::OneD(dkx), SpatialDims::OneD(nx)) => {
                let nx2 = if nx.is_even() {
                    (nx as f64) / 2.0
                } else {
                    (nx - 1) as f64 / 2.0
                };
                for ii in 0..nx {
                    k.push(vec![-nx2 * dkx + (ii as f64) * dkx]);
                }
                KSpace {
                    kspace: k,
                    num_channels: 1,
                }
            }
            (SpatialDims::TwoD(dkx, dky), SpatialDims::TwoD(nx, ny)) => {
                let nx2 = if nx.is_even() {
                    (nx as f64) / 2.0
                } else {
                    (nx - 1) as f64 / 2.0
                };
                let ny2 = if ny.is_even() {
                    (ny as f64) / 2.0
                } else {
                    (ny - 1) as f64 / 2.0
                };
                for jj in 0..ny {
                    for ii in 0..nx {
                        k.push(vec![
                            -nx2 * dkx + (ii as f64) * dkx,
                            -ny2 * dky + (jj as f64) * dky,
                        ]);
                    }
                }
                KSpace {
                    kspace: k,
                    num_channels: 2,
                }
            }
            (SpatialDims::ThreeD(dkx, dky, dkz), SpatialDims::ThreeD(nx, ny, nz)) => {
                let nx2 = if nx.is_even() {
                    (nx as f64) / 2.0
                } else {
                    (nx - 1) as f64 / 2.0
                };
                let ny2 = if ny.is_even() {
                    (ny as f64) / 2.0
                } else {
                    (ny - 1) as f64 / 2.0
                };
                let nz2 = if nz.is_even() {
                    (nz as f64) / 2.0
                } else {
                    (nz - 1) as f64 / 2.0
                };
                for kk in 0..nz {
                    for jj in 0..ny {
                        for ii in 0..nx {
                            k.push(vec![
                                -nx2 * dkx + (ii as f64) * dkx,
                                -ny2 * dky + (jj as f64) * dky,
                                -nz2 * dkz + (kk as f64) * dkz,
                            ]);
                        }
                    }
                }
                KSpace {
                    kspace: k,
                    num_channels: 3,
                }
            }
            _ => panic!("Wrong combination of things."),
        }
    }
}
