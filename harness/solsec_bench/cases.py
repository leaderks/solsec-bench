"""Discovery and metadata of benchmark cases."""

from __future__ import annotations

import tomllib
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
CASES_DIR = REPO_ROOT / "cases"


@dataclass(frozen=True)
class GroundTruth:
    file: str
    function: str
    summary: str


@dataclass(frozen=True)
class Case:
    id: str
    title: str
    vuln_class: str
    severity: str
    framework: str
    origin: str
    path: Path
    crate: str
    ground_truth: GroundTruth

    @property
    def vulnerable_source(self) -> Path:
        return self.path / "src" / "vulnerable.rs"

    @property
    def fixed_source(self) -> Path:
        return self.path / "src" / "fixed.rs"


def load_case(path: Path) -> Case:
    meta = tomllib.loads((path / "case.toml").read_text(encoding="utf-8"))
    cargo = tomllib.loads((path / "Cargo.toml").read_text(encoding="utf-8"))
    gt = meta["ground_truth"]
    return Case(
        id=meta["id"],
        title=meta["title"],
        vuln_class=meta["class"],
        severity=meta["severity"],
        framework=meta["framework"],
        origin=meta["origin"],
        path=path,
        crate=cargo["package"]["name"],
        ground_truth=GroundTruth(file=gt["file"], function=gt["function"], summary=gt["summary"]),
    )


def discover(cases_dir: Path = CASES_DIR) -> list[Case]:
    """Returns every case under `cases_dir`, sorted by id."""
    found = [load_case(p) for p in cases_dir.iterdir() if (p / "case.toml").is_file()]
    return sorted(found, key=lambda c: c.id)


def get(case_id: str, cases_dir: Path = CASES_DIR) -> Case:
    for case in discover(cases_dir):
        if case.id == case_id:
            return case
    raise KeyError(f"unknown case: {case_id}")
