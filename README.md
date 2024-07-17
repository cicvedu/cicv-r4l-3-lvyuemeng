![屏幕截图 2024-07-17 175308](https://github.com/user-attachments/assets/83f74cb1-8351-490d-a185-6018bdf16e63)# 作业报告

## 作业1：编译Linux内核

作业说明：

编译获得vmlinux。

**IMG:**

![屏幕截图 2024-07-17 175318](https://github.com/user-attachments/assets/0947c17d-c654-4f35-aa0d-61782e571907)

## 作业2：对Linux内核进行一些配置

作业说明：

1. 编译成内核模块，是在哪个文件中以哪条语句定义的?

**Answer:** 位于`Kbuild`文件中。
```
obj-m := r4l_e1000_demo.o
```

2. 该模块位于独立的文件夹内，却能编译成Linux内核模块，这叫做out-of-tree module，请分析它是如何与内核代码产生联系的?

**Answer:** 位于`Makefile`文件中。
```
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
```
obj-$(CONFIG_SAMPLE_RUST_HELLOWORLD)	+= rust_helloworld.o
```

`Kconfig`:
```
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

**Answer:**

`r4l_e1000_demo.rs`

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

`pci.rs`

	// In impl Device
	pub fn as_ptr(&self) -> *mut bindings::pci_dev {
            self.ptr
    	}

**IMG:**

![屏幕截图 2024-07-17 175308](https://github.com/user-attachments/assets/f4902afe-1d2d-4bab-ba7d-af08889e26df)





