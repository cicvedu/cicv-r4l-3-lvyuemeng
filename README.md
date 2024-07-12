[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/VsbltiDW)
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
