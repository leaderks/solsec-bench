# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed

- Case crates are named after the case id only (`ssb-NNNN`), so crate names no longer hint at the vulnerability class.
- The exploit prompt now gives the crate name to import.

### Added

- Case format: vulnerable program, reference fix, functional tests and an executable exploit test per case.
- SSB-0001 missing signer check and SSB-0002 missing owner check.
- Python harness to list and verify cases, render task prompts and score answers.
- Continuous integration for Rust (fmt, clippy, tests) and Python (ruff, pytest, case verification).
- SSB-0003 account type confusion: a user profile accepted where the config is expected.
- SSB-0004 PDA not re-derived: any signer can withdraw from another user's vault.
- SSB-0005 reinitialization: a funded vault can be initialized again with a new authority.
- SSB-0006 improper account closing: a closed ticket is revived in the same transaction and redeemed twice.
- SSB-0007 duplicate mutable accounts: a ledger transfer to itself doubles the balance.
- SSB-0008, first Anchor case: a vault withdraw without `has_one = authority`.
- `missing-authority-check` class in the taxonomy.
- Exploit prompts name the right entry point for Anchor programs (`entry`).
- ADOPTION.md to track third-party evaluations, external cases and integrations.
