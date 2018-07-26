// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! MRI

/// todo
pub struct RFSensitivityArray {
    /// todo
    pub array: Vec<RFSensitivity>,
}

impl RFSensitivityArray {
    pub fn new() -> Self {
        RFSensitivityArray { array: vec![] }
    }

    pub fn push(&mut self, rf: RFSensitivity) -> &mut Self {
        self.array.push(rf);
        self
    }
}

/// todo
pub struct RFSensitivity {
    /// todo
    pub sens: Vec<(f64, f64)>,
}

impl RFSensitivity {
    pub fn new(sens: Vec<(f64, f64)>) -> Self {
        RFSensitivity { sens }
    }
}
