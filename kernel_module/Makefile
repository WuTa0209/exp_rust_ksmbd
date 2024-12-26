obj-m += kernel_proc_add.o
kernel_module-objs += kernel_proc_add.o checker.o

CC = clang
LLVM_FLAGS = -fsanitize=undefined 

KDIR := /home/ubuntu/Desktop/workspace/kernel/linux_6.10/buildroot/output/build/linux-6.10.10
PWD := $(shell pwd)

all:
	$(MAKE) -C $(KDIR) M=$(PWD) modules CC=$(CC) CFLAGS_MODULE=$(LLVM_FLAGS) LDFLAGS_MODULE="$(LDFLAGS_MODULE)"

clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean
