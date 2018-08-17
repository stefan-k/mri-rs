// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! k-Space

use num::Integer;
use std::f64::consts::PI;
use SpatialDims;

/// A single k-space sample (one sample point in k-space)
pub type KSample = Vec<f64>;
/// A k-Space projection
pub type KProjection = Vec<KSample>;

/// Implement k-Space things!
pub trait KSpaceThings {
    /// One thing
    type KUnit;

    /// Thing 1
    fn add(&mut self, Self::KUnit) -> &mut Self;
    /// Thing 2
    fn sample_at(&self, usize) -> Self::KUnit;
    /// Thing 3
    fn set_sample(&mut self, usize, Self::KUnit) -> &mut Self;
    /// Thing 4
    fn num_channels(&self) -> usize;
    /// Thing 5
    fn num_samples(&self) -> usize;
    /// Thing 6
    fn num_units(&self) -> usize;
    /// Thing 7
    fn samples(&self) -> Vec<KSample>;
}

/// K-space defined as a set of projections
#[derive(Debug, Clone)]
pub struct KSpaceProjections {
    projections: Vec<KProjection>,
    num_channels: usize,
    num_samples: usize,
    num_projections: usize,
}

impl KSpaceProjections {
    /// Constructor
    pub fn new() -> Self {
        KSpaceProjections {
            projections: vec![],
            num_channels: 0,
            num_samples: 0,
            num_projections: 0,
        }
    }

    /// create radial trajectory (only 2D so far)
    pub fn radial(fov: f64, samples: usize, spokes: usize) -> Self {
        let dk = 1. / fov;
        let num_samples = samples * spokes;
        let mut k: Vec<KProjection> = Vec::with_capacity(spokes);
        // create single spoke
        let nx2 = if samples.is_even() {
            (samples as f64) / 2.0
        } else {
            (samples - 1) as f64 / 2.0
        };

        let mut spoke: Vec<KSample> = Vec::with_capacity(samples);
        for ii in 0..samples {
            spoke.push(vec![-nx2 * dk + (ii as f64) * dk, 0.0]);
        }

        for i in 0..spokes {
            let theta = (i as f64) * (PI / (spokes as f64));
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();
            let spoke_n: Vec<KSample> = spoke
                .iter()
                .map(|s| {
                    vec![
                        s[0] * cos_theta - s[1] * sin_theta,
                        s[0] * sin_theta + s[1] * cos_theta,
                    ]
                }).collect();
            k.push(spoke_n);
        }

        // create all spokes

        KSpaceProjections {
            projections: k,
            num_channels: 2,
            num_samples: num_samples,
            num_projections: spokes,
        }
    }
}

impl KSpaceThings for KSpaceProjections {
    type KUnit = KProjection;

    fn add(&mut self, proj: KProjection) -> &mut Self {
        let num_ch = proj[0].len();
        if self.num_channels == 0 {
            self.num_channels = proj[0].len();
        }
        assert!(num_ch == self.num_channels);
        self.projections.push(proj);
        self.num_projections += 1;
        self
    }

    fn sample_at(&self, idx: usize) -> KProjection {
        self.projections[idx].clone()
    }

    fn set_sample(&mut self, idx: usize, proj: KProjection) -> &mut Self {
        // assert!(self.num_channels == sample.len());
        self.projections[idx] = proj;
        self
    }

    fn num_channels(&self) -> usize {
        self.num_channels
    }

    fn num_samples(&self) -> usize {
        self.num_samples
    }

    fn num_units(&self) -> usize {
        self.num_projections
    }

    fn samples(&self) -> Vec<KSample> {
        self.projections
            .iter()
            .flat_map(|arr| arr.iter())
            .cloned()
            .collect()
    }
}

/// Representation of a k-space trajectory
#[derive(Debug, Clone)]
pub struct KSpace {
    /// A vector of k-space samples
    pub kspace: Vec<KSample>,
    /// Number of encoding channels
    num_channels: usize,
    /// Number of samples
    num_samples: usize,
}

impl KSpace {
    /// Constructor
    pub fn new() -> Self {
        KSpace {
            kspace: vec![],
            num_channels: 0,
            num_samples: 0,
        }
    }

    /// Create a Cartesian trajectory
    pub fn cartesian(fov: SpatialDims<f64>, samples: SpatialDims<usize>) -> Self {
        let dk = fov.invert();
        let num_samples = samples.product();
        let mut k: Vec<KSample> = Vec::with_capacity(num_samples);
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
                    num_samples: num_samples,
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
                    num_samples: num_samples,
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
                    num_samples: num_samples,
                }
            }
            _ => panic!("Wrong combination of things."),
        }
    }
}

impl KSpaceThings for KSpace {
    type KUnit = KSample;

    /// Add a single k-space sample point to an existing trajectory
    fn add(&mut self, sample: KSample) -> &mut Self {
        if self.num_channels == 0 {
            self.num_channels = sample.len();
        } else if self.num_channels != sample.len() {
            panic!("Wrong number of samples");
        }

        self.kspace.push(sample);
        self.num_samples += 1;
        self
    }

    /// Return sample at position `idx`
    fn sample_at(&self, idx: usize) -> KSample {
        self.kspace[idx].clone()
    }

    /// Set a sample at a specific position
    fn set_sample(&mut self, idx: usize, sample: KSample) -> &mut Self {
        assert!(self.num_channels == sample.len());
        self.kspace[idx] = sample;
        self
    }

    /// Return the number of channels
    fn num_channels(&self) -> usize {
        self.num_channels
    }

    /// Return the number of k-space samples
    fn num_samples(&self) -> usize {
        self.num_samples
    }

    /// Return the number of individual entities
    fn num_units(&self) -> usize {
        self.num_samples
    }

    fn samples(&self) -> Vec<KSample> {
        self.kspace.clone()
    }
}
