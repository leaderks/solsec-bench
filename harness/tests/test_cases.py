from solsec_bench import cases, tasks


def test_discovers_cases_with_metadata():
    found = cases.discover()
    ids = [c.id for c in found]
    assert ids == sorted(ids)
    assert {"SSB-0001", "SSB-0002"} <= set(ids)
    for case in found:
        assert case.vulnerable_source.is_file()
        assert case.fixed_source.is_file()
        assert (case.path / "tests" / "functional.rs").is_file()
        assert (case.path / "tests" / "exploit.rs").is_file()
        assert case.ground_truth.summary


def test_case_ids_match_directories():
    for case in cases.discover():
        assert case.path.name.startswith(case.id + "-")


def test_prompts_do_not_leak_answers():
    for case in cases.discover():
        fixed = case.fixed_source.read_text()
        exploit = (case.path / "tests" / "exploit.rs").read_text()
        for task in tasks.TASKS:
            prompt = tasks.render(case, task)
            assert case.ground_truth.summary not in prompt
            assert case.vuln_class not in prompt
            assert fixed not in prompt
            assert exploit not in prompt
            assert case.vulnerable_source.read_text().strip() in prompt
