---
name: Driver Request
about: Request a new hardware driver or driver feature
title: '[DRIVER] '
labels: enhancement, driver
assignees: ''
---

## Driver Request

### Hardware/Device Information

- **Device name**: (e.g., Intel I219-V Ethernet)
- **Manufacturer**: (e.g., Intel Corporation)
- **Device ID**: (PCI IDs, USB IDs, or other identifiers)
- **Hardware version**: (if applicable)

### Device Type

- [ ] Network (Ethernet/WiFi)
- [ ] Storage (NVMe/SATA/SCSI)
- [ ] USB (Host controller/Device)
- [ ] Graphics/Framebuffer
- [ ] Input (Keyboard/Mouse/Touch)
- [ ] Audio
- [ ] Serial/Console
- [ ] Other: ___________

### Target Platform

- [ ] QEMU virtualized (specify which device model)
- [ ] Specific bare metal hardware (list below)
- [ ] Generic/any platform

**Specific hardware target** (if applicable):
- Motherboard/SoC: 
- Relevant chipsets: 

### Technical Details

**Interface**:
- [ ] PCI/PCIe
- [ ] USB
- [ ] MMIO
- [ ] Port I/O
- [ ] ACPI
- [ ] Other: ___________

**Available Documentation**:
- Datasheet: (link or "proprietary/confidential")
- Programming manual: (link)
- Linux driver reference: (path or link)
- BSD driver reference: (path or link)
- OSDev wiki page: (link)

**Complexity Estimate**:
- [ ] Simple (single register interface, well-documented)
- [ ] Medium (interrupt handling, DMA, standard protocols)
- [ ] Complex (proprietary protocols, firmware required, timing-sensitive)

### Motivation/Use Case

Why is this driver needed? What use case does it enable?

```
Describe the specific use case, deployment scenario, or hardware
that requires this driver.
```

### Priority

- [ ] Blocker - Cannot proceed without this driver
- [ ] High - Significantly limits functionality
- [ ] Medium - Nice to have
- [ ] Low - Future consideration

### Additional Resources

- Links to existing open-source drivers for reference
- Hardware availability (can you provide remote access?)
- Test hardware available for developers?

## Checklist

- [ ] I have searched existing driver requests to avoid duplicates
- [ ] I have provided device identification information (PCI IDs, etc.)
- [ ] I have checked if documentation is publicly available
- [ ] I understand this is an experimental OS and driver support is limited
