## Description

Brief description of the changes in this PR.

Fixes # (issue number)

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring
- [ ] Build/CI improvement
- [ ] Security fix

## Component

- [ ] elminux-kernel (core kernel)
- [ ] elminux-hal (hardware abstraction)
- [ ] elminux-mm (memory manager)
- [ ] elminux-sched (scheduler)
- [ ] elminux-ipc (IPC system)
- [ ] elminux-drivers (driver framework)
- [ ] elminux-syscall (syscall ABI)
- [ ] elminux-std (standard library)
- [ ] epkg (package manager)
- [ ] elinit (init system)
- [ ] modsh (shell)
- [ ] Documentation
- [ ] Build system/Tooling
- [ ] Other: ___________

## Checklist

### Code Quality
- [ ] Code follows the project's style guidelines (`rustfmt`)
- [ ] `clippy` passes with no warnings (`make clippy`)
- [ ] Self-review of changes completed
- [ ] Code is commented, particularly in hard-to-understand areas
- [ ] Documentation updated (if applicable)

### Testing
- [ ] Tests added for new functionality
- [ ] Existing tests pass (`make test`)
- [ ] Manually tested in QEMU (if kernel change)
- [ ] No regressions in existing functionality

### Safety & Security
- [ ] All `unsafe` blocks have safety comments explaining invariants
- [ ] No undefined behavior introduced
- [ ] Privilege boundaries respected (kernel vs user space)
- [ ] Capability system semantics preserved (if IPC change)

### Commit Quality
- [ ] Commits are signed off (`git commit -s`)
- [ ] Commit messages follow conventional format:
  - `feat:` new feature
  - `fix:` bug fix
  - `docs:` documentation only
  - `style:` formatting
  - `refactor:` code change that neither fixes a bug nor adds a feature
  - `perf:` performance improvement
  - `test:` adding or correcting tests
  - `chore:` build process or auxiliary tool changes
- [ ] Commits are logically organized and squashed where appropriate

## Testing Performed

Describe the testing you performed:

```
- Build command used:
- QEMU version:
- Test cases run:
- Results:
```

## Performance Impact

- [ ] No performance impact expected
- [ ] Performance improvement (benchmarks below)
- [ ] Possible performance regression (justification below)

**Benchmarks** (if applicable):

```
Before: ___
After:  ___
Change: ___
```

## ABI Compatibility

- [ ] No ABI changes
- [ ] ABI change requires version bump (see ARCHITECTURE.md)
- [ ] New syscall added (number assigned, documented)
- [ ] IPC message format change (backward compatible?)

## Additional Notes

Any additional information, concerns, or notes for reviewers:

```
```

## Screenshots/Logs (if applicable)

```
Paste relevant serial output, screenshots, or logs here
```

---

**By submitting this PR, I certify that:**
- I have the right to submit this contribution under the GPL-3.0 license
- This contribution is my own original work or properly attributed
- I have followed the [Contributors Guide](../CONTRIBUTORS.md)
