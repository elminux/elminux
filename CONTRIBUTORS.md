# Contributing to Elminux

Thank you for your interest in contributing to Elminux! This document outlines
the contribution process and guidelines for participating in this project.

---

## Elminux Contributor Certificate (ECC) v1.0

By contributing to this project, you certify that:

```
Elminux Contributor Certificate
Version 1.0
Copyright (C) 2026 The Elminux Project contributors.
Everyone is permitted to copy and distribute verbatim copies of this
certificate document, but changing it is not allowed.

Elminux Contributor Certificate 1.0

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

---

## No Contributor License Agreement (CLA)

**Elminux does not require a Contributor License Agreement (CLA).**

We believe that the Elminux Contributor Certificate (ECC), combined with the
GPL-3.0 license, provides sufficient protection for both contributors and the
project. By signing off your commits with the ECC, you retain copyright to your
contributions while granting the project the rights needed to distribute them
under the project's license.

---

## How to Sign Off Your Commits

All commits must include a `Signed-off-by` line in the commit message. You can
add this automatically with Git:

```bash
git commit -s -m "Your commit message"
```

This will append a line like:

```
Signed-off-by: Your Name <your.email@example.com>
```

The name and email must match your Git configuration.

---

## Contribution Process

1. **Fork the repository** and create a feature branch from `dev`
2. **Make your changes** following the coding standards outlined in our documentation
3. **Write tests** for new functionality
4. **Ensure all tests pass** before submitting
5. **Commit with sign-off** using `git commit -s`
6. **Push to your fork** and submit a pull request to the `dev` branch

---

## Code Standards

- Follow Rust naming conventions and style guidelines
- Use `rustfmt` to format code
- Run `clippy` and address all warnings
- Document public APIs with rustdoc comments
- Write unit tests for kernel components where feasible
- Ensure `unsafe` blocks are documented with explicit safety invariants
- All `unsafe` outside `elminux-hal` requires maintainer approval

---

## Communication

- **GitHub Issues**: For bug reports, feature requests, and technical discussion
- **GitHub Discussions**: For general questions and community topics
- **Security issues**: See [SECURITY.md](./SECURITY.md) for responsible disclosure

---

## Questions?

If you have questions about contributing, please open a GitHub Discussion or
reach out to the maintainers.

Thank you for helping build Elminux.
