# 作业报告

## 作业1：编译Linux内核

作业说明：

编译获得vmlinux(提醒：编写或设置后必须重新编译)。

**IMG:**

![屏幕截图 2024-07-17 175318](https://github.com/user-attachments/assets/0947c17d-c654-4f35-aa0d-61782e571907)

## 作业2：对Linux内核进行一些配置

作业说明：

将提供的`src_e1000`的模块进行out of tree编译，并使其正常运行。

**Q:**

1. 编译成内核模块，是在哪个文件中以哪条语句定义的?

**Answer:** 位于`Kbuild`文件中。
```C
obj-m := r4l_e1000_demo.o
```

2. 该模块位于独立的文件夹内，却能编译成Linux内核模块，这叫做out-of-tree module，请分析它是如何与内核代码产生联系的?

**Answer:** 位于`Makefile`文件中。
```Makefile
KDIR ?= ../linux

default:
	$(MAKE) -C $(KDIR) M=$$PWD
```

- `KDIR ?= ../linux`表示如果`KDIR`没有被设置则默认为`../linux`
- `$(MAKE)`调用`make`指令编译
- `-C $(KDIR)`指定编译的内核目录
- `M==$$PWD`指定编译的模组目录为`$PWD`,即当前文件夹

**IMG:**
![屏幕截图 2024-07-12 151447](https://github.com/user-attachments/assets/e0b3dec8-dbd5-4ad4-b510-efd1988bdf88)
![屏幕截图 2024-07-12 155510](https://github.com/user-attachments/assets/0c97467f-894e-48d2-8a0f-bbdf1715c10c)

## 作业3：使用rust编写一个简单的内核模块并运行

作业说明：

创建`rust_helloworld.rs`并配置`Kconfig`,`Makefile`使其编译,最后将其编译的`.ko`文件复制入`rootf`中。

使用`insmod`执行，可看见`"Hello World from Rust module"`的输出。

**Answer:**

`Makefile`:

```Makefile
obj-$(CONFIG_SAMPLE_RUST_HELLOWORLD)	+= rust_helloworld.o
```

`Kconfig`:
```C
config SAMPLE_RUST_HELLOWORLD
	tristate "Print Helloworld in Rust"
	help
	  This option builds the print function by Rust.

	  To compile this as a module, choose M here:
	  the module will be called rust_helloworld.

	  If unsure, say N.
```

**IMG:**

![屏幕截图 2024-07-12 170842](https://github.com/user-attachments/assets/aab7c0a2-0147-463a-bd10-1b0c17bf1fa0)

## 作业4：为e1000网卡驱动添加remove代码

作业说明：

为e1000网卡添加remove功能，使其可以移除该模块。同时我们要求可以重新安装，恢复功能。

**Q:** 

作业5中的字符设备/dev/cicv是怎么创建的？它的设备号是多少？它是如何与我们写的字符设备驱动关联上的？

`rootfs/etc/init.d`

```sh
#!/bin/sh
mount -t proc none /proc
mount -t sysfs none /sys
/sbin/mdev -s
mknod /dev/cicv c 248 0
```

可知其在init.d中被初始化，主设备号:248,次设备号:0。在我们完善并注册字符设备后，通过设备号的对应形成关联。

**Answer:**

`r4l_e1000_demo.rs`
```rust
    fn remove(data: &Self::Data) {
        pr_info!("Rust for linux e1000 driver demo (remove)\n");

        let bars = data.bars;
        let pci_dev_ptr = data.pci_dev_ptr;

        unsafe {
            bindings::pci_clear_master(pci_dev_ptr);
            bindings::pci_release_selected_regions(pci_dev_ptr, bars);
            bindings::pci_disable_device(pci_dev_ptr);
        };

        data.e1000_hw_ops.as_arc_borrow().e1000_reset_hw();
    }

	// In fn probe
	Ok(Box::try_new(E1000DrvPrvData {
            // Must hold this registration, or the device will be removed.
            _netdev_reg: netdev_reg,
            bars,
            pci_dev_ptr: dev.as_ptr(),
            e1000_hw_ops: Arc::try_new(e1000_hw_ops)?,
	})?)
```

`pci.rs`
```rust
	// In impl Device
	pub fn as_ptr(&self) -> *mut bindings::pci_dev {
            self.ptr
    	}
```

`e1000_main.c`
```C
	// reference of 'e1000_remove' function
	pci_release_selected_regions(pdev, adapter->bars);

	disable_dev = !test_and_set_bit(__E1000_DISABLED, &adapter->flags);
	free_netdev(netdev);

	if (disable_dev)
		pci_disable_device(pdev);
```
  
**IMG:**

![屏幕截图 2024-07-17 175308](https://github.com/user-attachments/assets/f4902afe-1d2d-4bab-ba7d-af08889e26df)

## 作业5：注册字符设备

作业说明：

为`samples/rust/rust_chrdev.rs`补充`read`,`write`函数，使其`dev/cicv`可以完成基本的读写操作。


**Answer:**

更改配置：

```
Kernel hacking
  ---> Sample Kernel code
      ---> Rust samples
              ---> <*>Character device (NEW)
```

`rust_chrdev.rs`

```rust
    fn write(
        _this: &Self,
        _file: &file::File,
        _reader: &mut impl kernel::io_buffer::IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        let buf = &mut _this.inner.lock();
        let mut len = _reader.len();
        if len > GLOBALMEM_SIZE {
            len = GLOBALMEM_SIZE;
        }
        _reader.read_slice(&mut buf[..len])?;
        Ok(len)
    }

    fn read(
        _this: &Self,
        _file: &file::File,
        _writer: &mut impl kernel::io_buffer::IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        let data = &mut *_this.inner.lock();
        if _offset as usize >= GLOBALMEM_SIZE {
            return Ok(0);
        }
        _writer.write_slice(&data[_offset as usize..])?;
        Ok(data.len())
```

**IMG:**

![屏幕截图 2024-07-17 112625](https://github.com/user-attachments/assets/ff004826-90ff-4264-977a-a1c78703f3b5)








