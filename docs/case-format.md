# Case format

Each case lives in `cases/SSB-NNNN-short-name/` and is a member of the Cargo
workspace.

```
case.toml            metadata and hidden ground truth
Cargo.toml           crate manifest with a `fixed` feature
src/lib.rs           public API shared by both versions, selects the version
src/vulnerable.rs    the program under audit
src/fixed.rs         minimal reference fix
tests/common/mod.rs  helpers to build accounts
tests/functional.rs  behaviour every correct version keeps
tests/exploit.rs     passes only when the attack works
```

## `case.toml`

```toml
id = "SSB-0003"
title = "One sentence describing the flaw"
class = "missing-owner-check"   # see docs/taxonomy.md
severity = "critical"           # critical, high, medium, low
framework = "native"            # native, anchor, pinocchio
origin = "synthetic"            # or "incident" with a `reference` URL
added = "2026-10-16"

[ground_truth]
file = "src/vulnerable.rs"
function = "process_instruction"
summary = "What is wrong and how it is exploited."
```

## Rules

- **One vulnerability per case.** If you notice a second one, fix it in both
  versions or make it a separate case.
- **Minimal fix.** `fixed.rs` should differ from `vulnerable.rs` only where the
  bug is, so the diff documents the vulnerability.
- **Same public API.** Tests and prompts depend on `src/lib.rs` only.
- **Realistic code.** No comments or names that point at the bug.
- **Strong functional tests.** They must fail on a "fix" that simply rejects
  every call.
- **Incidents** must be public and fixed, with the post-mortem linked in
  `reference`.

Run `solsec-bench verify SSB-NNNN` before opening a pull request.
