# 作业报告

## 作业1：编译Linux内核

作业说明：
编译获得vmlinux

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

作业说明：创建`rust_helloworld.rs`并配置`Kconfig`,`Makefile`使其编译,最后将其编译的`.ko`文件复制入`rootf`中。

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


