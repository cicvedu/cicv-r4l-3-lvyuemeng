// SPDX-License-Identifier: GPL-2.0
//! Rust minimal sample.

use kernel::{driver, prelude::*};

module! {
  type: RustHelloWorld,
  name: "rust_helloworld",
  author: "whocare",
  description: "hello world module in rust",
  license: "GPL",
}

struct RustHelloWorld {}

impl kernel::Module for RustHelloWorld {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World from Rust module\n");
        Ok(RustHelloWorld {})
    }
}

impl driver::DeviceRemoval for RustHelloWorld {
    fn device_remove(&self) {
        drop(self);
    }
}

