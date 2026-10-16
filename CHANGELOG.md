# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Case format: vulnerable program, reference fix, functional tests and an executable exploit test per case.
- SSB-0001 missing signer check and SSB-0002 missing owner check.
- Python harness to list and verify cases, render task prompts and score answers.
- Continuous integration for Rust (fmt, clippy, tests) and Python (ruff, pytest, case verification).
