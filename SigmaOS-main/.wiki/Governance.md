# SigmaOS {crate_name} Wiki

## Crate Responsibility
Main repository with linking to all other repos, feature register (57 components tracked: 32 Integrated, 14 Prototype, 10 Proposed, 1 Supported), 103 design principles, 9 GitHub Wikis, CI configuration

## Design Principles (103 Principles)
The following 103 design principles are embedded across all SigmaOS crates, ensuring consistency and quality:

- **ABSTRACTION**
- **ARCHITECTURE**
- **ARRANGE-ACT-ASSERT**
- **AVOID FRAGMENTATION**
- **BLACKBOX**
- **BOOCH METHOD**
- **BOOTLOADER INTEGRATION**
- **BOY SCOUT RULE**
- **CACHE CONSCIOUS DESIGN**
- **CLEAR ABSTRACTION**
- **CODE REUSABILITY**
- **MAXIMISE COHESION**
- **COLLABORATION & SHARED RESPONSIBILITY**
- **COMMAND-QUERY SEPARATION**
- **COMMUNITY-ORIENTED DEVELOPMENT**
- **COMPONENT ISOLATION**
- **COMPOSITION OVER INHERITANCE**
- **CONTINUOUS IMPROVEMENT & LONG TERM SUSTAINABILITY**
- **MINIMISE COUPLING**
- **CROSS COMPILATION**
- **DEADLOCK AVOIDANCE**
- **DEFENSIVE PROGRAMMING**
- **DEMETER LAW**
- **DESIGN PATTERN**
- **DETERMINISTIC TESTING**
- **DEUTSCH LIMIT**
- **DIRECT DEVICE CONTROL**
- **DISCOVERABILITY**
- **DON'T REPEAT YOURSELF**
- **EFFICIENT SCHEDULING**
- **ENCAPSULATION**
- **FAIL-FAST SYSTEM**
- **FAST-ISOLATED-REPEATABLE-SELF VALIDATING-TIMELY**
- **FREE AND OPEN SOURCE SOFTWARE**
- **GALL'S LAW**
- **GENERAL RESPONSIBILITY ASSIGNMENT SOFTWARE PATTERNS**
- **GOVERNANCE AND DO-CRACY**
- **IF IT AIN'T BROKE, DON'T FIX IT**
- **INFORMATION HIDING**
- **INHERITANCE**
- **INSTRUMENTATION**
- **INTERFACE SEGREGATION**
- **INTERRUPT HANDLING DISCIPLINE**
- **INVERSION OF CONTROL**
- **KEEP IT SIMPLE STUPID**
- **LAYERED DESIGN**
- **LEAST ASTONISHMENT**
- **LEAST PRIVILEGE**
- **LISKOV SUBSTITUTION**
- **LOCALITY OF REFERENCE**
- **LOOSE COUPLING**
- **LOW LEVEL CONTROL**
- **LOW LEVEL LANGUAGES**
- **MANUAL MEMORY CONTROL**
- **MEMORY DISCIPLINE**
- **MEMORY SAFETY**
- **MERITOCRACY AND INCLUSIVITY**
- **MINIMAL KERNEL PHILOSOPHY**
- **MODULAR KERNEL DESIGN**
- **MODULARITY**
- **MULTIPLE REPOSITORY**
- **NINETY-NINETY RULE**
- **NORMALISATION**
- **OBJECT-ORIENTED PROGRAMMING**
- **OFFENSIVE PROGRAMMING**
- **OPEN-CLOSED**
- **OPEN STANDARD AND INTEROPERABILITY**
- **OPTIMISE FOR DELETION**
- **PLUG-IN DRIVER MODEL**
- **POLYMORPHISM**
- **PORTABILITY**
- **POWER OF TEN**
- **PRACTICAL LICENSING AWARENESS**
- **PRISTINE SOURCES**
- **PROCESS ISOLATION**
- **RACE-FREE DESIGN**
- **RESPECT & ETHICAL PARTICIPATION**
- **REFACTORING**
- **RULE OF THREE**
- **SAFE CONCURRENCY**
- **SECURE DEFAULTS**
- **SEPARATION OF CONCERNS**
- **SEPARATION OF MECHANISM & POLICY**
- **SINGLE RESPONSIBILITY, TRANSPARENCY & OPEN EXCHANGE**
- **SPEC-DRIVEN DEVELOPMENT**
- **OPEN/CLOSED, LISKOV SUBSTITUTION, INTERFACE SEGREGATION, DEPENDENCY INVERSION PRINCIPLE**
- **SOFTWARE SUPPORT PROGRAM JUST LIKE LINUX DISTROS**
- **SOFTWARE UPDATING PROGRAM JUST LIKE LINUX DISTROS**
- **STABLE APIs**
- **SINGLE REPOSITORY**
- **TALK IS CHEAP, SHOW ME THE CODE**
- **THEIRS**
- **UNIFORM ACCESS**
- **UNIFORM INFORMATION IN GITHUB REPO & GITHUB WIKI**
- **USER FREEDOM AND CONTROL**
- **VERSION CONTROL PRACTICES**
- **VIRTUAL MEMORY ABSTRACTION**
- **WORSE IS BETTER**
- **XTHEIRS**
- **YOU AREN'T GONNA NEED IT**
- **ZEN OF PYTHON**
- **ZERO-COPY**
- **ZERO-ONE-INFINITY**

## Canonical Repo Map
- SigmaOS-main: Main repository with linking to all other repos
- SigmaOS-sigma-system: Core system crate with runtime contract, SafeInstaller, security model, package ecosystem, release quality gates
- SigmaOS-sigma-kernel: Kernel core with boot information structures, BootInfo, FramebufferInfo, CPU/mode detection
- SigmaOS-sigma-bootloader: Bootloader with M1n1 bootflow, boot phases, boot menu, fallback/rescue flow
- SigmaOS-sigma-drivers: Hardware driver abstractions, QEMU virtio support (VirtioGPU/VirtioNetwork/VirtioSerial)
- SigmaOS-sigma-display: Display management with login flow, MainMenu (Start menu inspired by Linux Mint), Application launcher, DesktopLauncher
- SigmaOS-sigma-power: Power management with PowerManager, PowerMode (Performance/Powersave/Balanced/Auto), ACPI table parsing, SleepResume (auto-suspend/auto-resume with mem state)
- SigmaOS-sigma-persona: Persona/session management with SystemPersona (SovereignOmarchy/LinuxMintClassic/ArchMinimal), PersonaManager (save/restore lifecycle), PersonaSave (timestamped saves, delete/restore)
- SigmaOS-sigma-adjustments: Config schema and validation (GovernorMode, SystemAdjuster, ConfigSchema with migration from v0.1/v0.2), swappiness (0-100), vfs_cache_pressure (0-200), sysctl config generation

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
