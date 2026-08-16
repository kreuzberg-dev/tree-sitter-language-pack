# Reference-doc audit harness

Measures how many signatures in `docs-site/.../reference/api-<lang>.md` actually
appear in the binding a consumer compiles against. Frozen as a baseline so the
same measurement can be re-run after the generator is fixed.

```bash
python3 scripts/doc_audit/audit.py --json /tmp/after.json
diff <(cut -f1-4 scripts/doc_audit/baseline-members.txt) <(cut -f1-4 /tmp/after-members.txt)
```

## Columns

- `verbatim` — the documented signature equals a declaration line in the emitted
  binding, modulo whitespace and the body opener (`{`, `;`, `=>`, `do`). This is
  the number the fix has to move.
- `modulo` — same, after removing exactly one known per-language contract
  difference: java `throws`/`final`, dart `Future<>`/`static`/`async`, elixir's
  leading receiver arg, go's receiver. Where `verbatim` should land.
- `name-absent` — the documented member does not exist in the binding under any
  signature. These are fabrications, not mismatches.

## Baseline, 2026-08-16 (pre-fix, committed tree)

| lang | verbatim | modulo | name-absent |
|---|---|---|---|
| c | 1/77 | 1/77 | 45 |
| java | 0/77 | 65/77 | 10 |
| dart | 0/77 | 49/77 | 11 |
| elixir | 26/77 | 61/77 | 9 |
| csharp | 58/77 | 58/77 | 4 |
| go | 19/77 | 43/77 | 0 |

C's single match is `ts_pack_download_group` (`api-c.md:700` -> `ts_pack.h:2994`). It is the
degenerate shape — one `const char *`, scalar return, no handles, structs or out-params — which
is the one signature a generic template can emit and be accidentally right about.

## Why the member roster is frozen too

A fix that deletes fabricated members shrinks the denominator, so the ratio can
improve without any signature becoming more correct. `baseline-members.txt` is
the pre-fix roster; diff it alongside the counts.

## Controls

`audit.py` fails loudly if a language's negative control (`ZzzNotARealSymbol`)
matches, or if no documented name resolves at all — a zero-result sweep is
otherwise indistinguishable from a broken extractor. The harness was mutation
tested: corrupting one csharp signature moves `verbatim` 58 -> 57 and
`name-absent` 4 -> 5.

C is the case that needs those controls most: 1 match out of 77 is nearly
indistinguishable from a broken extractor. Its validity rests on that one match
being real, on the same code producing 58/77 for csharp, and on real header
lines indexing correctly.
