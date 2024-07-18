// SPDX-License-Identifier: GPL-2.0

//! Rust character device sample.

use core::result::Result::Err;

use kernel::prelude::*;
use kernel::sync::Mutex;
use kernel::task::Task;
use kernel::{chrdev, file};

const GLOBALMEM_SIZE: usize = 0x1000;

module! {
    type: RustChrdev,
    name: "completion_rust",
    author: "Learning Kernel Development",
    description: "Rust Completion Example",
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
        pr_info!("Rust Completion Example is invoked\n");
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
        pr_info!("Rust Completion Example (write)\n");

        let task = Task::current();

        pr_info!("process {} awakening the readers...\n", task.pid());

        let offset = _offset.try_into()?;
        let mut vec = _this.inner.lock();
        let len = core::cmp::min(_reader.len(), vec.len().saturating_sub(offset));
        _reader.read_slice(&mut vec[offset..][..len])?;

        Ok(len)
    }

    fn read(
        _this: &Self,
        _file: &file::File,
        _writer: &mut impl kernel::io_buffer::IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("Rust Completion Example (read)\n");

        let task = Task::current();

        pr_info!("process {} is going to sleep ...\n", task.pid());

        let offset = _offset.try_into()?;
        let vec = _this.inner.lock();
        let len = core::cmp::min(_writer.len(), vec.len().saturating_sub(offset));
        _writer.write_slice(&vec[offset..][..len])?;

        pr_info!("awoken process {}", task.pid());
        Ok(len)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl kernel::Module for RustChrdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Completion Example (init)\n");

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
        pr_info!("Rust Completion Example (exit)\n");
    }
}
