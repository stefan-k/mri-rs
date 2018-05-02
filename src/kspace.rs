// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! k-Space

type KChannel = Vec<f64>;

pub struct KSpace(Vec<KChannel>);

impl KSpace {
    pub fn new() -> Self {
        KSpace(vec![])
    }

    pub fn add_channel(&mut self, ch: KChannel) -> &mut Self {
        // todo: check dimensions
        self.0.push(ch);
        self
    }
}
