// Copyright 2019 Red Hat, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod types {
    pub type size_t = usize;

    pub mod raw {
        pub use ::std::os::raw::*;
    }
}

include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[test]
#[cfg(test)]
fn version_get_number() {
    unsafe {
        assert!(mbedtls_version_get_number() != 0);
    }
}
