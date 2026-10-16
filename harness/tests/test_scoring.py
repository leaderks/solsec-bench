import json

import pytest

from solsec_bench import cases, runner, scoring

SSB1 = cases.get("SSB-0001")


def test_detect_accepts_correct_answer():
    answer = json.dumps(
        {
            "class": "Missing Signer Check",
            "function": "process_instruction",
            "explanation": "authority never signs",
        }
    )
    assert scoring.score_detect(SSB1, answer).success


def test_detect_accepts_json_inside_prose():
    answer = 'Result:\n{"class": "missing-signer-check", "function": "process_instruction"}'
    assert scoring.score_detect(SSB1, answer).success


@pytest.mark.parametrize(
    "answer",
    [
        '{"class": "integer-overflow", "function": "process_instruction"}',
        '{"class": "missing-signer-check", "function": "withdraw"}',
        "not json",
        "[1, 2]",
    ],
)
def test_detect_rejects_wrong_answers(answer):
    assert not scoring.score_detect(SSB1, answer).success


@pytest.mark.cargo
def test_every_case_verifies():
    for case in cases.discover():
        failed = [c.name for c in runner.verify_case(case) if not c.ok]
        assert not failed, f"{case.id}: {failed}"


@pytest.mark.cargo
def test_reference_fix_scores_as_patch_and_original_does_not():
    assert scoring.score_patch(SSB1, SSB1.fixed_source.read_text()).success
    unchanged = scoring.score_patch(SSB1, SSB1.vulnerable_source.read_text())
    assert not unchanged.success
    assert unchanged.reason == "reference exploit still succeeds"


@pytest.mark.cargo
def test_reference_exploit_scores_and_trivial_test_does_not():
    exploit = (SSB1.path / "tests" / "exploit.rs").read_text()
    assert scoring.score_exploit(SSB1, exploit).success
    trivial = "#[test]\nfn always_passes() {}\n"
    result = scoring.score_exploit(SSB1, trivial)
    assert not result.success
    assert result.reason == "test also passes on the fixed program"


@pytest.mark.cargo
def test_patch_that_does_not_compile_is_reported():
    result = scoring.score_patch(SSB1, "this is not rust")
    assert not result.success
    assert result.reason == "does not compile"


@pytest.mark.cargo
def test_scoring_does_not_contaminate_reference_builds():
    # Regression: candidate builds once reused the reference target directory,
    # so a trivial candidate test replaced the real exploit binary.
    scoring.score_exploit(SSB1, "#[test]\nfn always_passes() {}\n")
    failed = [c.name for c in runner.verify_case(SSB1) if not c.ok]
    assert not failed
