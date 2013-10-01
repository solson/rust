// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() {
    assert_approx_eq!(1.0f64, 1.0);
    assert_approx_eq!(1.0000001f64, 1.0);
    assert_approx_eq!(1.0000001f64, 1.0, 1.0e-6);
    assert_approx_eq!(1.000001f64, 1.0, 1.0e-5);
}
