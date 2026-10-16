# Contributing

Thank you for helping make Solana programs safer.

## Ways to help

- **Add a case.** Follow [docs/case-format.md](docs/case-format.md).
- **Port a case** to Anchor or Pinocchio.
- **Improve the harness**: execution on LiteSVM, model adapters, scoring.
- **Report a problem** with a case: a second unintended bug, a weak test, a fix
  that breaks behaviour.

## Development setup

```sh
python -m venv .venv && . .venv/bin/activate
pip install -e '.[dev]'
```

Before opening a pull request, run what CI runs:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
ruff check . && ruff format --check .
pytest
solsec-bench verify
```

## Pull requests

- One case or one change per pull request.
- Describe what changes and why. For a new case, explain the vulnerability in
  the pull request, not in the code.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)
  (`feat:`, `fix:`, `docs:`, `case:` for new cases).
- By contributing you agree that your contribution is licensed under
  Apache-2.0.

## Responsible disclosure

Never submit an unfixed vulnerability in a live program. See [SECURITY.md](SECURITY.md).
