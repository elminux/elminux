# Elminux — Architecture

Version: 0.1.0-draft

---

## Overview

Elminux is a Debian minimal-based distribution with a custom:
- Hardware-constrained package selection and configuration
- Offline-first service and update model
- Multi-locale first-run experience
- Reproducible build pipeline targeting multiple hardware profiles

---

## Layer Model

```
┌──────────────────────────────────────────────────┐
│              User Application Layer               │
│  Firefox ESR │ LibreOffice │ VLC │ Geany │ ...   │
├──────────────────────────────────────────────────┤
│              Desktop Environment Layer            │
│  LXQt │ PCManFM │ LXTerminal │ SDDM (minimal)    │
├──────────────────────────────────────────────────┤
│              System Services Layer                │
│  NetworkManager │ avahi │ cups (optional)         │
├──────────────────────────────────────────────────┤
│              Security Hardening Layer             │
│  AppArmor │ auditd │ unattended-upgrades (LAN)    │
├──────────────────────────────────────────────────┤
│              Base OS Layer                        │
│  Debian minimal │ systemd │ apt │ glibc           │
├──────────────────────────────────────────────────┤
│              Hardware / Firmware Layer            │
│  x86_64 │ aarch64 (RPi 4+) │ UEFI + BIOS legacy  │
└──────────────────────────────────────────────────┘
```

---

## Boot Architecture

```
BIOS/UEFI
    │
    ▼
GRUB2 (supports both legacy BIOS and UEFI)
    │
    ├── Partition A (active OS)   ─── read-only squashfs overlay
    └── Partition B (standby OS)  ─── receives OTA updates
            │
            ▼
    /data (persistent, user data)
    ext4, optional LUKS2 encryption
```

- **Legacy BIOS support** is mandatory — recycled hardware often lacks UEFI
- **A/B partition scheme** enables atomic updates with rollback
- **/data partition** persists across OS updates
- **squashfs overlay** on OS partition minimizes disk wear (important for SD cards)

---

## Hardware Profiles

| Profile ID | Target | RAM | Storage | Desktop |
|---|---|---|---|---|
| `x86-min` | Recycled PC, Pentium era | 512MB | 8GB HDD | LXQt |
| `x86-std` | Recycled PC, Core2 era+ | 1GB | 16GB | LXQt |
| `arm64-rpi` | Raspberry Pi 4/5 | 1GB | 16GB SD | LXQt |
| `live-usb` | Any x86_64, ephemeral | 512MB | 2GB USB | LXQt |
| `headless` | Server/kiosk, no display | 256MB | 4GB | None |

Each profile maps to a build configuration controlling:
- Kernel flavour (standard vs RT vs low-latency)
- Package selection (slim vs full)
- Desktop presence
- Default swappiness and memory tuning

---

## Network Model

```
Default state: OFFLINE (fully functional)
    │
    ▼
LAN mode: local network, no internet required
    │
    ▼
WAN mode: internet available
    ├── Software updates via apt (standard Debian mirrors)
    └── Offline mirror sync (LAN apt-cacher-ng relay)
```

### Offline Software Distribution
- ISO ships with a **local apt mirror** of curated packages
- `apt-cacher-ng` can run on one node to serve an entire LAN
- USB-based package transfer supported (sneakernet workflow)

---

## Update Architecture

```
┌─────────────────────────────────┐
│     Upstream Debian Security    │
└────────────────┬────────────────┘
                 │ (when WAN available)
                 ▼
┌─────────────────────────────────┐
│   LMIC Linux Update Server      │
│   (curated, tested, signed)     │
└────────────────┬────────────────┘
                 │
        ┌────────┴────────┐
        │                 │
        ▼                 ▼
   Direct WAN         LAN relay
   (apt sources)    (apt-cacher-ng)
        │                 │
        └────────┬────────┘
                 ▼
         Device: Partition B
         (atomic apply, reboot to activate)
         (rollback on failed boot)
```

---

## Build System

```
build/
├── Dockerfile.builder         # Reproducible Debian-based build env
├── config/
│   ├── profiles/              # Per-hardware profile configs
│   │   ├── x86-min.conf
│   │   ├── x86-std.conf
│   │   ├── arm64-rpi.conf
│   │   ├── live-usb.conf
│   │   └── headless.conf
│   ├── package-lists/         # Curated apt package lists per profile
│   ├── hooks/                 # lb_config hooks (live-build)
│   └── includes/              # Files injected into rootfs
├── scripts/
│   ├── hardening.sh           # Post-install hardening (idempotent)
│   ├── locale-setup.sh        # Multi-locale first-run config
│   └── partition-layout.sh    # A/B + /data partition setup
└── Makefile                   # Top-level build targets
```

**Build toolchain:** Debian `live-build` (lb_config/lb_build)
- Industry standard for Debian-derived distros
- Produces: ISO, raw disk image, OTA bundle
- Fully scriptable and CI-compatible

---

## Locale & Language Architecture

First-run wizard collects:
1. Region → maps to locale set + timezone + keyboard
2. Language → installs language pack, sets LANG/LC_*
3. Input method → IBus profile for CJK/Indic/Arabic scripts

Initial supported locale groups:
- **Africa:** sw (Swahili), ha (Hausa), am (Amharic), fr-AF
- **Asia:** id (Indonesian), tl (Filipino), hi (Hindi), bn (Bengali)
- **Pacific:** tpi (Tok Pisin), fj (Fijian)
- **Latin America:** es, pt-BR, qu (Quechua)
- **Base:** en

All locale data ships on ISO — no internet required for language setup.

---

## Security Model

| Control | Implementation |
|---|---|
| AppArmor | Enabled, enforcing mode, upstream Debian profiles |
| Firewall | ufw, default-deny inbound |
| Updates | apt, signed (Debian keyring), LAN-relay capable |
| Audit | auditd, default rules |
| Encryption | LUKS2 optional (installer choice, /data partition) |
| Secure Boot | UEFI shim + signed kernel (x86 profiles) |
| No telemetry | Zero outbound data collection, verifiable in build |

---

## Versioning

SemVer 2.0.0:
- `MAJOR` — incompatible hardware profile or base Debian version change
- `MINOR` — new hardware profile, new locale group, new default application
- `PATCH` — security patches, bug fixes, configuration corrections

Artifact format: `elminux-{VERSION}-{PROFILE}.iso`
Example: `elminux-1.0.0-x86-std.iso`

Debian base tracks: **Debian Stable** (current: Bookworm 12.x)
Upgrade to next Debian Stable = MAJOR version bump.
