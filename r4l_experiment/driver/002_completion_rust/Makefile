ifneq ($(KERNELRELEASE),)

# In kbuild context
module-objs := rust_completion.o	
obj-m := rust_completion.o

CFLAGS_hello_world.o := -DDEBUG
else
KDIR := ../../../linux
PWD := $(shell pwd)

all:
	$(MAKE) LLVM=1 -C $(KDIR)  M=$(PWD) modules

.PHONY: clean
clean:
	rm -f *.ko *.o .*.cmd .*.o.d *.mod *.mod.o *.mod.c *.symvers *.markers *.unsigned *.order *~
endif