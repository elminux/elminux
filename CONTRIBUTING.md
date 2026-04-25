# Contributing to Elminux

## Rust Toolchain

`rust-toolchain.toml` is pinned to a dated nightly (`nightly-YYYY-MM-DD`).

### Bump policy
- Only bump when a new feature or fix requires a newer compiler.
- Before bumping, build and test the kernel (`cargo build --package elminux-kernel` and QEMU smoke test).
- Update the date in `rust-toolchain.toml` and document the reason in the commit message.
- CI must pass on the new nightly before the PR is merged.

## Submodules

The `userland/modsh` directory is currently tracked in-tree and is **not** a Git submodule yet. To convert it to a pinned submodule:

1. Remove the in-tree directory and add the external `modsh` repository as a submodule:
   ```bash
   rm -rf userland/modsh
   git submodule add https://github.com/modsh-shell/modsh userland/modsh
   git submodule update --init
   ```
   `modsh` is a separate project that predates Elminux and is still under active development.

2. Pin the submodule to a specific commit SHA by checking out the desired commit inside `userland/modsh` and committing the submodule pointer in the parent repository.

3. Bump procedure:
   - `cd userland/modsh`
   - `git fetch origin`
   - `git checkout <new_commit_sha>`
   - `cd ../..`
   - `git add userland/modsh`
   - `git commit -m "Bump modsh to <short_sha>"`
