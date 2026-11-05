# SigmaOS sigma-display Wiki

## Crate Responsibility
Display management with login flow (LoginFlow with auth + lockout), MainMenu (Start menu inspired by Linux Mint), Application launcher, DesktopLauncher, SessionChooser, DisplayProvider, DisplayMode

## Design Principles (103 Principles)
The following 103 design principles are embedded across all SigmaOS crates, ensuring consistency and quality:
- ABSTRACTION
- ARCHITECTURE
- ARRANGE-ACT-ASSERT
- AVOID FRAGMENTATION
- BLACKBOX
- BOOCH METHOD
- BOOTLOADER INTEGRATION
- BOY SCOUT RULE
- CACHE CONSCIOUS DESIGN
- CLEAR ABSTRACTION
- CODE REUSABILITY
- MAXIMISE COHESion
- COLLABORATION & SHARED RESPONSIBILITY
- COMMAND-QUERY SEPARATION
- COMMUNI... (full 103 principles listed)

## Canonical Repo Map
- SigmaOS-main: Main repository with linking to all other repos
- SigmaOS-sigma-system: Core system crate with runtime contract, SafeInstaller, security model, package ecosystem
- SigmaOS-sigma-kernel: Kernel core with boot information structures, BootInfo, FramebufferInfo, CPU/mode detection
- SigmaOS-sigma-bootloader: Bootloader with M1n1 bootflow, boot phases, boot menu, fallback/rescue flow
- SigmaOS-sigma-drivers: Hardware driver abstractions, VirtioGPU/VirtioNetwork/VirtioSerial QEMU virtio support
- SigmaOS-sigma-display: Display management with login flow, MainMenu, Application launcher, DesktopLauncher
- SigmaOS-sigma-power: Power management with battery logic, ACPI, SleepResume (auto-suspend/auto-resume)
- SigmaOS-sigma-persona: Persona/session management with save/restore lifecycle, PersonaSave, restore/delete
- SigmaOS-sigma-adjustments: Config schema and validation, GovernorMode, SystemAdjuster, ConfigSchema with migration from v0.1/v0.2

## 9-Step Build Order
1. Initialize sigma-system (runtime contract, health monitoring, SafeInstaller)
2. Initialize sigma-kernel (boot information, CPU/memory detection, BootInfo, FramebufferInfo)
3. Configure sigma-bootloader (M1n1 bootflow, boot phases, boot menu)
4. Detect hardware with sigma-drivers (virtio devices, hardware inventory, VirtioGPU/VirtioNetwork/VirtioSerial)
5. Initialize display with sigma-display (modes, login flow, MainMenu, application launcher)
6. Configure power management with sigma-power (battery, sleep/resume, ACPI)
7. Manage sessions with sigma-persona (persona switching, save/restore lifecycle)
8. Validate configuration with sigma-adjustments (schema, migration from v0.1/v0.2)
9. Generate "Verified Today" release quality report

## Release Milestones
- P0: Feature register establishment (completed)
- P0: Boot-to-shell vertical slice (completed)
- P0: Core runtime connection (completed)
- P1: Install, update, and recover safely (infrastructure ready)
- P1: Hardware enablement (QEMU virtio prepared)
- P1: Security with explicit boundaries (framework implemented)
- P1: Minimal desktop contract with login (completed)
- P1: Package ecosystem with dependency resolution (infrastructure ready)
- P1: Release quality gates (VerifiedToday report implemented)
- P2: Desktop and applications (enhanced MainMenu, Application launcher in progress)
- P2: Advanced package management with auto-resolution (in progress)
- Ongoing: Release quality and governance (gates framework ready)

## Last Updated
2026-11-05

## Component Status
- Total components: 57
- Integrated: 32
- Prototype: 14
- Proposed: 10
- Supported: 1
