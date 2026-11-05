# SigmaOS

## A modular Rust-based operating system inspired by Linux Mint and Omarchy Linux

**Status**: Early development - alpha stage

**Current Focus**: P0 - establish what works (feature register tracking, boot infrastructure, runtime contract)

**Quick Start (QEMU)**:
```bash
# Build and run in QEMU
cd sigma-kernel && cargo build --release
# ... build instructions would go here
```

**Documentation**: 
- GitHub Wiki: https://github.com/AaryanSinghChauhan09/SigmaOS-main/wiki
- Feature Register: [view feature-register.json](feature-register.json)
- All 103 design principles are documented across 9 GitHub Wikis

**8 Modular Crates**:
- `sigma-system`: Core runtime contract and health monitoring
- `sigma-kernel`: Kernel core with boot information structures
- `sigma-bootloader`: M1n1 bootloader with UEFI/UBOOT/Coreboot support
- `sigma-drivers`: Hardware driver abstractions (QEMU virtio support)
- `sigma-display`: Display management with login flow
- `sigma-power`: Power management with battery logic
- `sigma-persona`: Persona/session management
- `sigma-adjustments`: Config schema and validation

**Roadmap Progress**:
- Feature register: 57 components tracked (16 Integrated, 17 Prototype, 23 Proposed, 1 Supported)
- 8 crates compile successfully
- 103 design principles embedded across all crates
- GitHub Wikis consolidated with governance documentation

**Status Signals** (from feature-register.json):
- `Integrated`: Core structures and types that are implemented
- `Prototype`: Components implemented but needing validation
- `Proposed`: Planned components or needing more work
- `Supported`: The feature register itself

**Contact**: GitHub Issues for contributions and feedback

*This project is in active development. See the GitHub Wiki for detailed documentation, design principles, and roadmap status.*
