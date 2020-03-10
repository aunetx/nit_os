# Todo

## Short-term

- [ ] support multitasking — `+++`

- [ ] change drivers structure — `+++`

- [ ] add real drivers — `+++`

  - [ ] graphics
    - [ ] `VGA` modes
      - [x] `3h`: 80x25 alphanimerics 16-colors — `+++`
      - [ ] `13h`: 320x200 linear 256-colors — `++`
      - [ ] `12h`: 640x480 planar 16-colors? — `-`
    - [ ] permit `VGA` switching — `++`
    - [ ] implement characters drawing in graphics mode — `+`
    - [ ] use `GPU` acceleration — `-`

  - [ ] storage devices
    - [ ] `ATA` — `+`
    - [ ] `AHCI` — `+`
    - [ ] `floppy` — `-`

  - [ ] `I/O`
    - [ ] `PS/2 keyboard` — `+`
    - [ ] mouse — `+`
    - [x] `serial ports` — `+++`

  - [ ] controllers
    - [ ] `PCI` — `++`
    - [ ] `USB` — `+`
    - [ ] `PCIe` — `-`

  - [ ] `ACPI` — `++`

  - [ ] interrupts
    - [ ] `PIC` — `+++`
      - [x] basic implementation
      - [ ] full implementation
    - [ ] `APIC` — `++`
      - [ ] timer — `++`
    - [x] `CMOS` — `+`

  - [ ] networking
    - [ ] any ethernet chip — `-`

  - [ ] audio — `--`

## Long-term

- [ ] change allocator to a more powerful one

- [ ] implement filesystem

- [ ] support multiprocessing

- [ ] remove arbitrary crates (`bootloader`, `x86_64`...)

- [ ] allow userspace processes

- [ ] support rust's `std`

- [ ] support `gcc` etc
