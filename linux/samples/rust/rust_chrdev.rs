// SPDX-License-Identifier: GPL-2.0

//! Rust character device sample.

use core::ops::Deref;
use core::result::Result::Err;
use std::cmp::min;

use kernel::prelude::*;
use kernel::sync::Mutex;
use kernel::{chrdev, file};

const GLOBALMEM_SIZE: usize = 0x1000;

module! {
    type: RustChrdev,
    name: "rust_chrdev",
    author: "Rust for Linux Contributors",
    description: "Rust character device sample",
    license: "GPL",
}

static GLOBALMEM_BUF: Mutex<[u8; GLOBALMEM_SIZE]> = unsafe { Mutex::new([0u8; GLOBALMEM_SIZE]) };

struct RustFile {
    #[allow(dead_code)]
    inner: &'static Mutex<[u8; GLOBALMEM_SIZE]>,
}

#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
        Ok(Box::try_new(RustFile {
            inner: &GLOBALMEM_BUF,
        })?)
    }

    fn write(
        _this: &Self,
        _file: &file::File,
        _reader: &mut impl kernel::io_buffer::IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        let mut buf = _this.inner.lock().deref();

        let len = min(buf.len() - _offset as usize, _reader.len());
        if len <= 0 {
            return Ok(0);
        }

        _reader.read_slice(&mut buf[_offset as usize..][..len])?;
        Ok(len)
    }

    fn read(
        _this: &Self,
        _file: &file::File,
        _writer: &mut impl kernel::io_buffer::IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        let mut data = *_this.inner.lock().deref();

        let len = min(data.len() - _offset as usize, _writer.len());
        if len <= 0 {
            return Ok(0);
        }

        _writer.write_slice(&mut data[_offset as usize..][..len])?;
        Ok(len)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl kernel::Module for RustChrdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}
