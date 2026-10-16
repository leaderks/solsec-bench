# Vulnerability classes

Classes focus on failure modes created by Solana's account model. Labels are
kebab-case and used as `class` in `case.toml`.

| Class | Description |
| --- | --- |
| missing-signer-check | An account that must authorize the action is not required to sign |
| missing-owner-check | Account data is trusted without checking which program owns the account |
| account-type-confusion | An account of one type is accepted where another type is expected |
| arbitrary-cpi | The program invokes a program id supplied by the caller without checking it |
| pda-misuse | A PDA is not re-derived, uses weak seeds, or is shared across users |
| duplicate-mutable-accounts | The same account is passed twice where two distinct accounts are assumed |
| reinitialization | An already initialized account can be initialized again |
| improper-account-closing | A closed account can be revived or reused |
| arithmetic | Overflow, underflow, or precision loss in amounts or prices |
| oracle-manipulation | Prices or data from an external account can be manipulated within a transaction |

The list grows with the benchmark. Propose a new class in an issue first.
