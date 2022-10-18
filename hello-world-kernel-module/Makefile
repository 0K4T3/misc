ifneq ($(KERNELRELEASE),)
	obj-m := hello.o
else
	KERNELDIR ?= /lib/modules/$(shell uname -r)/build
	PWD := $(shell pwd)

default:
	$(MAKE) -C $(KERNELDIR) M=$(PWD) modules
endif

install:
	insmod hello.ko

uninstall:
	rmmod hello

clean:
	rm Module.symvers hello.ko hello.o hello.mod hello.mod.c hello.mod.o modules.order
