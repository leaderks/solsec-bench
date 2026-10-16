# Security policy

SolSec Bench contains intentionally vulnerable programs. They exist to measure
security tools and AI models. **Never deploy any program from `cases/` to a
public cluster.**

## What belongs in this repository

- Synthetic vulnerabilities written for the benchmark.
- Reproductions of incidents that are already public **and** fixed, with a link
  to the public post-mortem.

## What does not

Do not open an issue or pull request about a vulnerability in a live program
that is not yet fixed. Report it privately to the affected project first, using
its `security.txt` or bug bounty program, and wait for the fix and public
disclosure.

## Reporting a problem with this repository

If you find a security problem in the harness itself (for example a way for an
evaluated model to escape the sandbox or tamper with scores), use GitHub's
private vulnerability reporting on this repository ("Security" tab, "Report a
vulnerability"). You will get an answer within 7 days.
