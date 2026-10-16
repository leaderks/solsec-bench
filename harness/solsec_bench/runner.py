"""Builds and runs case tests with cargo, in place or in an isolated copy."""

from __future__ import annotations

import os
import shutil
import subprocess
import tempfile
import time
from collections.abc import Iterator
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path

from .cases import REPO_ROOT, Case

WORKSPACE_FILES = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml")

# Isolated copies share the same workspace-relative paths as the real cases,
# so cargo would treat their artifacts as interchangeable. They get their own
# target directory so a candidate build never overwrites a reference build.
MAIN_TARGET = REPO_ROOT / "target"
ISOLATED_TARGET = REPO_ROOT / "target" / "isolated"


@dataclass(frozen=True)
class TestRun:
    test: str
    fixed: bool
    passed: bool
    compiled: bool
    output: str


def run_test(
    case_path: Path,
    test: str,
    *,
    fixed: bool = False,
    target_dir: Path = MAIN_TARGET,
    timeout: int = 900,
) -> TestRun:
    """Runs one integration test target (`functional` or `exploit`) of a case."""
    cmd = ["cargo", "test", "--quiet", "--manifest-path", str(case_path / "Cargo.toml")]
    cmd += ["--test", test]
    if fixed:
        cmd += ["--features", "fixed"]
    env = {**os.environ, "CARGO_TARGET_DIR": str(target_dir)}
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, env=env)
    output = proc.stdout + proc.stderr
    compiled = "could not compile" not in output and "error[E" not in output
    return TestRun(
        test=test, fixed=fixed, passed=proc.returncode == 0, compiled=compiled, output=output
    )


@contextmanager
def isolated_copy(case: Case) -> Iterator[Path]:
    """Yields the path of a throwaway copy of `case` inside a minimal workspace.

    Candidate patches and exploits are written into the copy, never into the
    repository.
    """
    with tempfile.TemporaryDirectory(prefix=f"{case.id}-") as tmp:
        root = Path(tmp)
        for name in WORKSPACE_FILES:
            src = REPO_ROOT / name
            if src.is_file():
                shutil.copy2(src, root / name)
        dest = root / "cases" / case.path.name
        shutil.copytree(case.path, dest)
        # Fresh timestamps force cargo to rebuild the copy instead of reusing
        # an artifact built from another candidate.
        now = time.time()
        for path in dest.rglob("*"):
            os.utime(path, (now, now))
        yield dest


@dataclass(frozen=True)
class Check:
    name: str
    ok: bool
    detail: str


def verify_case(case: Case) -> list[Check]:
    """Checks the four properties every case must hold."""
    expectations = [
        ("functional tests pass on vulnerable", "functional", False, True),
        ("functional tests pass on fixed", "functional", True, True),
        ("exploit succeeds on vulnerable", "exploit", False, True),
        ("exploit is blocked on fixed", "exploit", True, False),
    ]
    checks = []
    for name, test, fixed, expect_pass in expectations:
        run = run_test(case.path, test, fixed=fixed)
        ok = run.compiled and run.passed == expect_pass
        checks.append(Check(name=name, ok=ok, detail="" if ok else run.output[-2000:]))
    return checks
