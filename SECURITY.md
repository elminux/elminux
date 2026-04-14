# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0.0 | :x:                |

Pre-1.0 releases are experimental and do not receive security support. We
recommend waiting for v1.0.0 for production use.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:

**[security@elminux.org](mailto:security@elminux.org)**

You should receive an acknowledgment within **48 hours**. If you do not
receive a response within that timeframe, please follow up to ensure your
report was received.

### What to Include

When reporting a vulnerability, please include:

- **Description**: A clear description of the vulnerability
- **Affected versions**: Which versions or commit ranges are affected
- **Impact**: The potential security impact (e.g., privilege escalation, data leak)
- **Steps to reproduce**: Detailed instructions to reproduce the issue
- **Proof of concept**: If available, a minimal demonstration
- **Suggested fix**: If you have ideas for remediation

### PGP Key

For sensitive reports, you may encrypt your message using our security team's
PGP key (available at [elminux.org/security/pgp-key.txt](https://elminux.org/security/pgp-key.txt)).

## Disclosure Policy

Elminux follows a **coordinated disclosure** model:

1. **Initial report** received and acknowledged
2. **Assessment** period (up to 7 days) to validate and assess severity
3. **Remediation** development (timeline varies by severity)
4. **Patch preparation** and testing
5. **Coordinated disclosure** after patch is available

### Timeline

| Severity | Response Time | Fix Target | Disclosure |
|----------|--------------|------------|------------|
| Critical | 24 hours | 14 days | After patch release |
| High | 48 hours | 30 days | After patch release |
| Medium | 7 days | 90 days | After patch release |
| Low | 14 days | Next release | After patch release |

## CVE Assignment

Elminux participates in the CVE Program. Valid vulnerabilities will receive
CVE identifiers according to the following SLA:

- **Critical/High severity**: CVE assigned within 7 days of patch release
- **Medium/Low severity**: CVE assigned within 30 days of patch release

CVE entries will be published through MITRE and posted to the
[NVD](https://nvd.nist.gov/).

## Security Advisories

Security advisories will be published:

- On our website: [elminux.org/security/advisories](https://elminux.org/security/advisories)
- Via GitHub Security Advisories
- To the [oss-security](https://seclists.org/oss-security/) mailing list for
critical vulnerabilities

Each advisory includes:
- CVE identifier (if assigned)
- Affected versions
- Severity rating (CVSS 3.1)
- Description of the vulnerability
- Patch or workaround information
- Credits to the reporter

## Acknowledgments

We will publicly acknowledge reporters who responsibly disclose vulnerabilities,
unless they request to remain anonymous.

## Security Best Practices for Users

- Run Elminux in virtualized/test environments until v1.0.0
- Apply updates promptly when security fixes are released
- Monitor [elminux.org/security](https://elminux.org/security) for advisories
- Review the [Architecture Documentation](./ARCHITECTURE.md) for security model
details

## Scope

This security policy covers:
- The Elminux kernel and core components
- System call implementations
- Memory management code
- IPC mechanisms
- Driver framework
- Boot process and initialization

Third-party drivers and user-space applications distributed separately may
have their own security policies.

## Bug Bounty

Elminux does not currently operate a bug bounty program. We recognize security
researchers through public acknowledgments and CVE credits.
