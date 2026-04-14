# Elminux

> A community-owned, general-purpose Linux distribution built for
> Low- and Middle-Income Country (LMIC) realities: constrained hardware,
> unreliable connectivity, diverse locales, and community maintainability.

---

## Problem Statement

Mainstream Linux distributions are designed for hardware and connectivity
conditions that do not reflect the majority of the world's computer users.
LMIC users face: recycled/aging hardware, intermittent or zero internet,
limited technical support, and underrepresented languages. No community-owned,
general-purpose distribution is built specifically for these constraints.

Elminux fills that gap.

---

## Design Principles

1. **Offline-first** — full usability with zero internet connectivity
2. **Constrained hardware** — minimum 512MB RAM, 8GB storage, Pentium-era x86_64 + ARM
3. **Driver coverage** — Debian base ensures broadest recycled hardware support
4. **Multi-locale** — first-class support for underrepresented LMIC languages
5. **Minimal footprint** — fast boot, low idle RAM, no bloat
6. **Community maintainable** — no single vendor dependency, forkable by any community
7. **Transparent** — SBOM published with every release, reproducible builds

---

## Target Hardware

| Profile | Minimum Spec |
|---|---|
| x86_64 (recycled PC/laptop) | 512MB RAM, 8GB HDD, Pentium 4 era+ |
| x86_64 (recommended) | 1GB RAM, 16GB SSD |
| aarch64 (SBC) | Raspberry Pi 4, 1GB RAM, 16GB SD |
| Live USB | Any x86_64 with 2GB USB |

---

## Default Software

| Category | Included |
|---|---|
| Browser | Firefox ESR (offline docs cache) |
| Office | LibreOffice (minimal profile) |
| Media | VLC |
| Text editor | Geany |
| File manager | PCManFM |
| Terminal | LXTerminal |
| Desktop | LXQt |
| Package manager | apt (Debian) |

All default software is offline-capable.

---

## License

GPL-3.0 — see [LICENSE](./LICENSE)

Bundled software retains their respective upstream licenses (Debian policy compliant).

---

## Status

`v0.1.0-alpha` — Scaffold / Pre-development

---

## Community

- Website: https://elminux.org
- GitHub: https://github.com/elminux
- Forum: TBD
- Matrix/IRC: TBD
- Mailing list: TBD
