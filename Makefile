# Elminux Makefile
# Build targets for kernel development

.PHONY: all qemu iso test clippy doc clean limine

# Configuration
KERNEL_CRATE := elminux-kernel
LIMINE_VERSION := 7.0.0
BUILD_DIR := target/x86_64-unknown-none
ISO_DIR := build/iso
LIMINE_DIR := limine

# Default target
all: $(BUILD_DIR)/release/elminux-kernel

# Build kernel
$(BUILD_DIR)/release/elminux-kernel:
	cargo build --release -p $(KERNEL_CRATE)

$(BUILD_DIR)/debug/elminux-kernel:
	cargo build -p $(KERNEL_CRATE)

# Run in QEMU (serial output to stdio)
qemu: $(BUILD_DIR)/debug/elminux-kernel
	qemu-system-x86_64 \
		-cpu qemu64,+apic,+acpi \
		-smp 1 \
		-m 512M \
		-serial stdio \
		-display none \
		-no-reboot \
		-no-shutdown \
		-kernel $<

# Run release build in QEMU
qemu-release: $(BUILD_DIR)/release/elminux-kernel
	qemu-system-x86_64 \
		-cpu qemu64,+apic,+acpi \
		-smp 1 \
		-m 512M \
		-serial stdio \
		-display none \
		-no-reboot \
		-no-shutdown \
		-kernel $<

# Create bootable ISO
iso: $(BUILD_DIR)/release/elminux-kernel limine
	mkdir -p $(ISO_DIR)/boot/limine
	mkdir -p $(ISO_DIR)/boot/EFI/BOOT
	cp $< $(ISO_DIR)/boot/elminux-kernel.elf
	cp limine.cfg $(ISO_DIR)/boot/limine/
	cp $(LIMINE_DIR)/limine-bios.sys $(ISO_DIR)/boot/limine/
	cp $(LIMINE_DIR)/limine-bios-cd.bin $(ISO_DIR)/boot/limine/
	cp $(LIMINE_DIR)/limine-uefi-cd.bin $(ISO_DIR)/boot/limine/
	cp $(LIMINE_DIR)/BOOTX64.EFI $(ISO_DIR)/boot/EFI/BOOT/
	xorriso -as mkisofs \
		-b boot/limine/limine-bios-cd.bin \
		-no-emul-boot \
		-boot-load-size 4 \
		-boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part \
		--efi-boot-image \
		--protective-msdos-label \
		$(ISO_DIR) \
		-o elminux.iso
	$(LIMINE_DIR)/limine bios-install elminux.iso

# Run headless tests in QEMU
test: $(BUILD_DIR)/release/elminux-kernel
	qemu-system-x86_64 \
		-cpu qemu64,+apic,+acpi \
		-smp 1 \
		-m 512M \
		-serial file:test.log \
		-display none \
		-no-reboot \
		-no-shutdown \
		-kernel $< \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04
	@echo "Test log saved to test.log"

# Run clippy on all crates
clippy:
	cargo clippy --workspace -- -D warnings

# Generate documentation
doc:
	cargo doc --workspace --no-deps --document-private-items

# Clean build artifacts
clean:
	cargo clean
	rm -rf build/
	rm -f elminux.iso
	rm -f test.log

# Download and setup Limine bootloader
limine:
	@if [ ! -d "$(LIMINE_DIR)" ]; then \
		git clone --depth=1 --branch=v$(LIMINE_VERSION)-binary \
			https://github.com/limine-bootloader/limine.git \
			$(LIMINE_DIR); \
	fi

# Development helpers
fmt:
	cargo fmt --all

check:
	cargo check --workspace
