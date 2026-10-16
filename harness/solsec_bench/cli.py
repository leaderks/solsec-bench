"""Command-line entry point: `solsec-bench <command>`."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict
from pathlib import Path

from . import cases, runner, scoring, tasks


def _cmd_list(_: argparse.Namespace) -> int:
    for case in cases.discover():
        print(f"{case.id}  {case.vuln_class:<26} {case.severity:<9} {case.title}")
    return 0


def _cmd_verify(args: argparse.Namespace) -> int:
    selected = [cases.get(i) for i in args.ids] if args.ids else cases.discover()
    failures = 0
    for case in selected:
        for check in runner.verify_case(case):
            print(f"[{'ok' if check.ok else 'FAIL'}] {case.id}: {check.name}")
            if not check.ok:
                failures += 1
                print(check.detail, file=sys.stderr)
    return 1 if failures else 0


def _cmd_prompt(args: argparse.Namespace) -> int:
    print(tasks.render(cases.get(args.id), args.task))
    return 0


def _cmd_score(args: argparse.Namespace) -> int:
    answer = Path(args.answer).read_text(encoding="utf-8")
    score = scoring.SCORERS[args.task](cases.get(args.id), answer)
    print(json.dumps(asdict(score), indent=2))
    return 0 if score.success else 1


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="solsec-bench", description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    sub.add_parser("list", help="list cases").set_defaults(func=_cmd_list)

    verify = sub.add_parser("verify", help="check that cases behave as specified")
    verify.add_argument("ids", nargs="*", help="case ids (default: all)")
    verify.set_defaults(func=_cmd_verify)

    prompt = sub.add_parser("prompt", help="print the prompt for a task")
    prompt.add_argument("id")
    prompt.add_argument("--task", choices=tasks.TASKS, required=True)
    prompt.set_defaults(func=_cmd_prompt)

    score = sub.add_parser("score", help="score a model answer stored in a file")
    score.add_argument("id")
    score.add_argument("--task", choices=tasks.TASKS, required=True)
    score.add_argument("--answer", required=True, help="path to the answer file")
    score.set_defaults(func=_cmd_score)
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
