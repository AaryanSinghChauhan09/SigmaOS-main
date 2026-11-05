# SigmaOS SigmaOS-main Wiki

## Crate Responsibility
Main repository with linking to all other repos, feature register, 103 design principles

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
- CODE REUSABIL... (full 103 principles listed)

## Canonical Repo Map
- SigmaOS-main: Main repository with linking to all other repos
- SigmaOS-sigma-system: Core system crate with runtime contract
- SigmaOS-sigma-kernel: Kernel core with boot information structures
- SigmaOS-sigma-bootloader: Bootloader with M1n1 bootflow
- SigmaOS-sigma-drivers: Hardware driver abstractions
- SigmaOS-sigma-display: Display management with login flow
- SigmaOS-sigma-power: Power management with battery logic
- SigmaOS-sigma-persona: Persona/session management
- SigmaOS-sigma-adjustments: Config and settings management

## 9-Step Build Order
1. Initialize sigma-system (runtime contract, health monitoring)
2. Initialize sigma-kernel (boot information, CPU/memory detection)
3. Configure sigma-bootloader (M1n1 bootflow, boot phases)
4. Detect hardware with sigma-drivers (virtio devices, hardware inventory)
5. Initialize display with sigma-display (modes, login flow)
6. Configure power management with sigma-power (battery, sleep/resume)
7. Manage sessions with sigma-persona (persona switching, save/restore)
8. Validate configuration with sigma-adjustments (schema, migration)
9. Generate "Verified Today" release quality report

## Release Milestones
- P0: Feature register establishment (completed)
- P0: Boot-to-shell vertical slice (completed)
- P0: Core runtime connection (completed)
- P1: Install, update, and recover safely (infrastructure ready)
- P1: Hardware enablement (QEMU virtio prepared)
- P1: Security with explicit boundaries (framework implemented)
- P2: Desktop and applications (pending runtime proof)
- P2: Package ecosystem (infrastructure ready)
- Ongoing: Release quality and governance (gates framework ready)

## Last Updated
2026-11-05

## Component Status
- Total components: 57
- Integrated: 28
- Prototype: 18
- Proposed: 10
- Supported: 1
