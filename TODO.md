# Elminux — TODO

Standard: OSS SDLC / DevSecOps
Versioning: SemVer 2.0.0
Base: Debian Stable (Bookworm 12.x)
Build toolchain: live-build
Status tags: [ ] todo | [x] done | [~] in-progress | [!] blocked

---

## v0.1.0 — Governance & Foundation

### 1. Legal & Governance
- [ ] 1.1 Commit LICENSE (GPL-3.0)
- [ ] 1.2 Write CONTRIBUTORS.md (DCO — Developer Certificate of Origin, no CLA)
- [ ] 1.3 Write CODE_OF_CONDUCT.md (Contributor Covenant 2.1)
- [ ] 1.4 Write SECURITY.md (vulnerability disclosure policy + CVE response SLA)
- [x] 1.5 Confirm project name: Elminux
- [x] 1.6 Register domain (elminux.org secured)
- [x] 1.7 Set up GitHub org (github.com/elminux secured)
- [ ] 1.8 Configure branch protection (main, dev)
- [ ] 1.9 Write issue templates (bug, feature-request, hardware-support, locale-request)
- [ ] 1.10 Write PR template with checklist

### 2. Requirements & Research
- [ ] 2.1 Compile hardware support matrix from Debian HCL + recycled PC surveys
- [ ] 2.2 Survey existing LMIC Linux users (min 3 regions: Africa, SE Asia, Latin America)
- [ ] 2.3 Define formal package inclusion criteria
  - [ ] 2.3.1 Must be in Debian main (no non-free by default)
  - [ ] 2.3.2 Must be offline-capable
  - [ ] 2.3.3 Must function on minimum hardware profile
- [ ] 2.4 Define threat model document
- [ ] 2.5 Define initial locale group (see ARCHITECTURE.md)
- [ ] 2.6 Competitive audit: Endless OS, Raspberry Pi OS, Trisquel, Sugar
---

## v0.2.0 — Build System

### 3. Build Infrastructure
- [ ] 3.1 Set up Dockerfile.builder (Debian Bookworm, live-build installed)
- [ ] 3.2 Create Makefile with build targets
  - [ ] 3.2.1 `make build PROFILE=x86-min`
  - [ ] 3.2.2 `make build PROFILE=x86-std`
  - [ ] 3.2.3 `make build PROFILE=arm64-rpi`
  - [ ] 3.2.4 `make build PROFILE=live-usb`
  - [ ] 3.2.5 `make build PROFILE=headless`
- [ ] 3.3 Write lb_config base configuration
  - [ ] 3.3.1 Debian Bookworm stable as base
  - [ ] 3.3.2 GRUB2 bootloader (BIOS + UEFI dual support)
  - [ ] 3.3.3 systemd init
  - [ ] 3.3.4 apt sources: Debian main + security only (no contrib/non-free by default)
- [ ] 3.4 Write per-profile package lists
  - [ ] 3.4.1 x86-min package list (absolute minimum viable desktop)
  - [ ] 3.4.2 x86-std package list (comfortable daily use)
  - [ ] 3.4.3 arm64-rpi package list (RPi firmware + standard)
  - [ ] 3.4.4 live-usb package list (ephemeral, persistence optional)
  - [ ] 3.4.5 headless package list (no desktop, server tools)
- [ ] 3.5 Implement A/B partition layout script
- [ ] 3.6 Implement /data persistent partition setup
- [ ] 3.7 Produce bootable ISO from CI (x86-std first)

### 4. CI/CD Pipeline
- [ ] 4.1 Set up GitHub Actions
  - [ ] 4.1.1 Build job (matrix: x86-min, x86-std, arm64-rpi, live-usb)
  - [ ] 4.1.2 Shellcheck lint job
  - [ ] 4.1.3 SBOM generation (syft, CycloneDX format)
  - [ ] 4.1.4 CVE scan (grype against SBOM)
  - [ ] 4.1.5 ISO checksum + GPG signing
  - [ ] 4.1.6 Artifact upload to release
- [ ] 4.2 Set up release pipeline (tag → build → sign → publish)
- [ ] 4.3 Set up reproducibility verification (hash comparison across two independent builds)

---

## v0.3.0 — Hardware Support Layer

### 5. Kernel & Drivers
- [ ] 5.1 Use Debian linux-image-generic (Bookworm default)
- [ ] 5.2 Add firmware-linux-free package
- [ ] 5.3 Evaluate firmware-linux-nonfree inclusion
  - [ ] 5.3.1 Policy decision: separate non-free profile vs default-include
  - [ ] 5.3.2 Document non-free driver list + rationale
- [ ] 5.4 Test boot on minimum hardware targets
  - [ ] 5.4.1 Pentium 4 era x86_64 with 512MB RAM
  - [ ] 5.4.2 Core 2 Duo era x86_64 with 1GB RAM
  - [ ] 5.4.3 Raspberry Pi 4 (2GB)
- [ ] 5.5 Configure low-memory kernel parameters (x86-min profile)
  - [ ] 5.5.1 vm.swappiness tuning
  - [ ] 5.5.2 Disable unused kernel modules
  - [ ] 5.5.3 zram swap (compressed RAM swap, critical for 512MB)

### 6. Performance Tuning (Constrained Hardware)
- [ ] 6.1 Configure zram on all profiles (2x RAM compressed swap)
- [ ] 6.2 Configure preload / readahead tuning
- [ ] 6.3 Disable unnecessary systemd services by default
- [ ] 6.4 Configure LXQt compositor (disable effects on x86-min)
- [ ] 6.5 Benchmark boot time on minimum hardware (target: <30s)
- [ ] 6.6 Benchmark idle RAM on minimum hardware (target: <150MB)

---

## v0.4.0 — Desktop & Applications

### 7. Desktop Environment
- [ ] 7.1 Install LXQt (latest stable in Bookworm)
- [ ] 7.2 Configure SDDM (minimal theme, low resource)
- [ ] 7.3 Create Elminux default theme
  - [ ] 7.3.1 Wallpaper (community-contributed, license-clear)
  - [ ] 7.3.2 Icon theme (lightweight)
  - [ ] 7.3.3 GTK/Qt theme (coherent, low-resource)
- [ ] 7.4 Configure default panel layout (taskbar, application menu)
- [ ] 7.5 Configure application menu (category-organized, translated)

### 8. Default Applications
- [ ] 8.1 Firefox ESR
  - [ ] 8.1.1 Pre-configure offline documentation bookmark set
  - [ ] 8.1.2 Disable telemetry by default
  - [ ] 8.1.3 Configure uBlock Origin pre-installed
- [ ] 8.2 LibreOffice (minimal profile: Writer, Calc, Impress only)
  - [ ] 8.2.1 Disable Java dependency (use built-in Base alternative)
  - [ ] 8.2.2 Pre-configure for low RAM (disable Java, reduce cache)
- [ ] 8.3 VLC (video/audio)
- [ ] 8.4 Geany (text editor, lightweight IDE-capable)
- [ ] 8.5 PCManFM (file manager)
- [ ] 8.6 LXTerminal
- [ ] 8.7 Gpicview or gThumb (image viewer)
- [ ] 8.8 Evince (PDF viewer)
- [ ] 8.9 Transmission-gtk (torrent — important for offline content distribution)

### 9. Offline Content Bundle (Optional ISO Layer)
- [ ] 9.1 Kiwix integration (offline Wikipedia, offline content)
  - [ ] 9.1.1 Pre-bundle Wikipedia mini (text-only, top languages)
  - [ ] 9.1.2 Bundle local Kiwix library browser
- [ ] 9.2 Khan Academy Lite or KA-Lite (offline education, optional)
- [ ] 9.3 Document offline content update via USB procedure

---

## v0.5.0 — Security Hardening

### 10. OS Hardening
- [ ] 10.1 Enable AppArmor (enforcing mode, upstream Debian profiles)
- [ ] 10.2 Configure ufw (default-deny inbound, allow established)
- [ ] 10.3 Configure auditd (default Debian rules)
- [ ] 10.4 Disable unused network services by default
- [ ] 10.5 Configure automatic security updates (unattended-upgrades)
  - [ ] 10.5.1 Security-only, not full upgrades
  - [ ] 10.5.2 Configurable: on/off, LAN-relay aware
- [ ] 10.6 Write idempotent hardening.sh script
- [ ] 10.7 Validate against CIS Debian Linux Benchmark (Level 1)

### 11. Update Security
- [ ] 11.1 Verify apt uses Debian signed repos by default
- [ ] 11.2 Implement GPG verification for any Elminux overlay packages
- [ ] 11.3 Implement A/B rollback on failed boot
- [ ] 11.4 Document air-gap update procedure (USB apt offline)
- [ ] 11.5 Configure apt-cacher-ng for LAN relay deployment

---

## v0.6.0 — Locale & Language Layer

### 12. First-Run Wizard
- [ ] 12.1 Build first-run wizard (Python + GTK or shell + dialog)
  - [ ] 12.1.1 Step 1: Language selection
  - [ ] 12.1.2 Step 2: Region / timezone
  - [ ] 12.1.3 Step 3: Keyboard layout
  - [ ] 12.1.4 Step 4: Input method (IBus for non-Latin scripts)
  - [ ] 12.1.5 Step 5: User account creation
  - [ ] 12.1.6 Step 6: Network setup (optional)
- [ ] 12.2 All locale data ships on ISO (no internet required)

### 13. Locale Packages
- [ ] 13.1 Africa group: sw, ha, am, fr (Africa locales)
- [ ] 13.2 Asia group: id, tl, hi, bn
- [ ] 13.3 Pacific group: tpi, fj
- [ ] 13.4 Latin America group: es, pt-BR, qu
- [ ] 13.5 Base: en-US, en-GB
- [ ] 13.6 Test each locale: fonts, input method, number/date formatting
- [ ] 13.7 Community locale contribution process documented

---

## v0.7.0 — Installer

### 14. Installer
- [ ] 14.1 Evaluate: Debian d-i vs Calamares vs custom shell
  - Decision rationale: Calamares preferred (Qt, LXQt-native, LMIC-friendly)
- [ ] 14.2 Build Calamares installer configuration
  - [ ] 14.2.1 Welcome screen (multi-language)
  - [ ] 14.2.2 Locale/keyboard page
  - [ ] 14.2.3 Disk layout page (guided + manual)
  - [ ] 14.2.4 LUKS2 encryption toggle (optional, warned on perf impact)
  - [ ] 14.2.5 User creation page
  - [ ] 14.2.6 Summary + install
  - [ ] 14.2.7 Post-install: A/B partition setup + /data partition
- [ ] 14.3 Text-mode fallback installer (for headless profile)
- [ ] 14.4 Live session support (run without installing)
- [ ] 14.5 Persistence option for live-usb profile

---

## v0.8.0 — Documentation & Community

### 15. Documentation
- [ ] 15.1 User guide (installation, first use, offline features)
- [ ] 15.2 Administrator guide (updates, user management, LAN relay setup)
- [ ] 15.3 Hardware procurement guide (LMIC-sourced, tested components)
- [ ] 15.4 Contributor guide (build system, locale contribution, packaging)
- [ ] 15.5 Air-gap update procedure
- [ ] 15.6 LAN relay (apt-cacher-ng) setup guide
- [ ] 15.7 SBOM interpretation guide
- [ ] 15.8 Translate docs to: fr, es, pt-BR, id, sw (priority order)

### 16. Community Infrastructure
- [ ] 16.1 Set up project website (elminux.org, static, GitHub Pages)
- [ ] 16.2 Set up Matrix room (primary community channel)
- [ ] 16.3 Set up mailing list (for governance/announcements)
- [ ] 16.4 Set up forum (Discourse or lightweight alternative)
- [ ] 16.5 Define regional maintainer program
  - [ ] 16.5.1 Africa maintainer(s)
  - [ ] 16.5.2 Southeast Asia maintainer(s)
  - [ ] 16.5.3 Latin America maintainer(s)
  - [ ] 16.5.4 Pacific maintainer(s)

---

## v0.9.0 — Testing & Validation

### 17. Test Suite
- [ ] 17.1 Boot test matrix (all profiles, all hardware targets)
- [ ] 17.2 Application smoke tests (all default apps)
- [ ] 17.3 Locale tests (each language group)
- [ ] 17.4 Offline functionality tests (zero network, all features)
- [ ] 17.5 Security regression tests
  - [ ] 17.5.1 ufw ruleset validation
  - [ ] 17.5.2 AppArmor enforcement verification
  - [ ] 17.5.3 No outbound telemetry verification
- [ ] 17.6 A/B rollback test (deliberate boot failure)
- [ ] 17.7 Hardware stress test on 512MB RAM profile
- [ ] 17.8 Install test on minimum disk (8GB HDD)

### 18. SBOM & CVE Workflow
- [ ] 18.1 Generate SBOM on every release build (CycloneDX format)
- [ ] 18.2 Publish SBOM with each release artifact
- [ ] 18.3 Define CVE response SLA
  - Critical: 7 days
  - High: 30 days
  - Medium: 90 days
- [ ] 18.4 Automate CVE scan on SBOM in CI (grype)
- [ ] 18.5 Public CVE tracker page on project website

---

## v1.0.0 — Stable Release

### 19. Pre-release
- [ ] 19.1 Beta release to regional maintainers
- [ ] 19.2 Field test: minimum 3 LMIC deployments (different regions)
- [ ] 19.3 Integrate field feedback
- [ ] 19.4 External security review (academic or NGO partner)
- [ ] 19.5 Freeze installer + config interface (stability contract)
- [ ] 19.6 Finalize all documentation + translations

### 20. Release
- [ ] 20.1 Tag v1.0.0
- [ ] 20.2 Publish signed ISO artifacts (all profiles)
- [ ] 20.3 Publish SBOM
- [ ] 20.4 Announce (DistroWatch submission, OSS forums, NGO networks, elminux.org)
- [ ] 20.5 Define LTS support window (minimum 3 years security patches)
- [ ] 20.6 Align LTS window with Debian Bookworm LTS (until ~2028)

---

## Post v1.0.0 Backlog

### Profiles
- [ ] B.1 Education profile (GCompris, Scratch, offline Khan Academy)
- [ ] B.2 Medical profile (GNU Health, Orthanc)
- [ ] B.3 Agricultural profile (offline soil, crop, weather tools)
- [ ] B.4 Sovereignty profile (airgap-capable, SBOM-verifiable, audit-ready — targets gov/institutional deployment)

### Hardware & Connectivity
- [ ] B.5 Solar/low-power optimization (suspend tuning, display power)
- [ ] B.6 Mesh networking profile
  - [ ] B.6.1 LoRa / Meshtastic first-class support
  - [ ] B.6.2 WiFi mesh (batman-adv or similar)
  - [ ] B.6.3 Connectivity-independent LAN-over-mesh
  - [ ] B.6.4 Use cases: disaster response, maritime, rural, conflict zones

### Local AI Layer
- [ ] B.7 Ship curated local model runner (llama.cpp-based)
  - [ ] B.7.1 Target: functional on 2GB RAM
  - [ ] B.7.2 Use cases: offline assistant, translation, document summarizer
  - [ ] B.7.3 No cloud dependency, no telemetry, verifiable weights
  - [ ] B.7.4 Model selection policy (size, license, language coverage)

### Governance & Ecosystem
- [ ] B.8 Foundation/legal entity formation for long-term governance
- [ ] B.9 Official Debian derivative registration (Debian derivatives census)
- [ ] B.10 Partnership with NGOs for hardware distribution programs
- [ ] B.11 Sovereignty certification pathway (target: ANSSI-compatible, EU digital sovereignty frameworks)
