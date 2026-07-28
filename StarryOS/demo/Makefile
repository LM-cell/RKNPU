CROSS_COMPILE_riscv64 ?= /opt/musl/riscv64-linux-musl-cross/bin/riscv64-linux-musl-
CROSS_COMPILE_aarch64 ?= /opt/musl/aarch64-linux-musl-cross/bin/aarch64-linux-musl-

DISK_IMG ?= /home/inkbottle/othersrc/npu/StarryOS/arceos/disk.img
MOUNT_DIR ?= /tmp/starryos-disk-img-mnt
SUDO ?= sudo

SUPPORTED_ARCHES := riscv64 aarch64
ARCHES ?= $(SUPPORTED_ARCHES)
INSTALL_ARCH ?= aarch64
BUILD_DIR := build

CPPFLAGS += -I.
COMMON_CFLAGS += -O2 -g -static -std=c11 -Wall -Wextra
CFLAGS_riscv64 += -march=rv64gc -mabi=lp64d
CFLAGS_aarch64 += -march=armv8-a
LDFLAGS += -static

APP_SRCS := $(filter-out libnpu.c,$(wildcard *.c))
APP_BINS := $(patsubst %.c,%,$(APP_SRCS))

.PHONY: all clean install list $(foreach arch,$(SUPPORTED_ARCHES),all-$(arch) install-$(arch))

all: $(foreach arch,$(ARCHES),all-$(arch))

list:
	@for arch in $(ARCHES); do \
		for bin in $(APP_BINS); do \
			printf '%s/%s\n' "$$arch" "$$bin"; \
		done; \
	done

define DEFINE_ARCH_RULES
CC_$(1) := $$(CROSS_COMPILE_$(1))gcc
STRIP_$(1) := $$(CROSS_COMPILE_$(1))strip
ARCH_BUILD_DIR_$(1) := $(BUILD_DIR)/$(1)
LIB_OBJ_$(1) := $$(ARCH_BUILD_DIR_$(1))/libnpu.o
ARCH_BINS_$(1) := $$(patsubst %,$$(ARCH_BUILD_DIR_$(1))/%,$$(APP_BINS))

all-$(1): $$(ARCH_BINS_$(1))

$$(ARCH_BUILD_DIR_$(1)):
	mkdir -p $$@

$$(LIB_OBJ_$(1)): libnpu.c libnpu.h | $$(ARCH_BUILD_DIR_$(1))
	$$(CC_$(1)) $$(CPPFLAGS) $$(COMMON_CFLAGS) $$(CFLAGS_$(1)) -c -o $$@ $$<

$$(ARCH_BUILD_DIR_$(1))/%: %.c $$(LIB_OBJ_$(1)) libnpu.h | $$(ARCH_BUILD_DIR_$(1))
	$$(CC_$(1)) $$(CPPFLAGS) $$(COMMON_CFLAGS) $$(CFLAGS_$(1)) -o $$@ $$< $$(LIB_OBJ_$(1)) $$(LDFLAGS)
	$$(STRIP_$(1)) $$@

install-$(1): all-$(1)
	@set -eu; \
	if [ ! -f "$(DISK_IMG)" ]; then \
		echo "disk image not found: $(DISK_IMG)"; \
		exit 1; \
	fi; \
	if [ "$$(id -u)" -eq 0 ]; then \
		SUDO_CMD=; \
	elif $(SUDO) -n true >/dev/null 2>&1; then \
		SUDO_CMD='$(SUDO)'; \
	else \
		echo "install requires root to mount $(DISK_IMG)"; \
		echo "run: sudo make install INSTALL_ARCH=$(1)"; \
		exit 1; \
	fi; \
	mkdir -p "$(MOUNT_DIR)"; \
	mounted=0; \
	if ! mountpoint -q "$(MOUNT_DIR)"; then \
		$$SUDO_CMD mount -o loop "$(DISK_IMG)" "$(MOUNT_DIR)"; \
		mounted=1; \
	fi; \
	cleanup() { \
		if [ "$$mounted" -eq 1 ]; then \
			$$SUDO_CMD umount "$(MOUNT_DIR)"; \
		fi; \
	}; \
	trap cleanup EXIT INT TERM; \
	$$SUDO_CMD mkdir -p "$(MOUNT_DIR)/test"; \
	for bin in $(APP_BINS); do \
		$$SUDO_CMD install -m 0755 "$(BUILD_DIR)/$(1)/$$bin" "$(MOUNT_DIR)/test/$$bin"; \
		$$SUDO_CMD install -m 0755 "$(BUILD_DIR)/$(1)/$$bin" "$(MOUNT_DIR)/test/$$bin.$(1)"; \
		echo "installed $$bin ($(1)) -> $(DISK_IMG):/test/$$bin"; \
		echo "installed $$bin ($(1)) -> $(DISK_IMG):/test/$$bin.$(1)"; \
	done
endef

$(foreach arch,$(SUPPORTED_ARCHES),$(eval $(call DEFINE_ARCH_RULES,$(arch))))

install: install-$(INSTALL_ARCH)

clean:
	rm -rf $(BUILD_DIR)
