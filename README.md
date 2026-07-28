# dobby

Structural merge for generated data.
`dobby merge --id id base.json export.json` grafts a fresh export into the committed version of a file:
content comes from the export, order and formatting come from the base file.
The git diff then shows the real changes instead of the generator's reshuffling.

## Problem

Generators of structured data - schema and settings exporters, CMSs, config dumps -
emit valid JSON in a nondeterministic order.
Under a line-based diff a commit with two real edits turns into thousands of noisy lines:
review goes blind, merges conflict over nothing, history loses its meaning.

Canonicalization does not fix it.
Array order is often semantically significant - the order of form fields, the sequence of steps -
so sorting changes what the consumer of the data sees.

## How it works

dobby splits the sources of truth.
The committed file owns order and style, the incoming export owns content.
Array elements are matched by a stable id regardless of position,
and the resulting delta (added / removed / modified) is spliced into the base file without rewriting it.
Untouched regions stay byte for byte: element and key order, indentation, line breaks, number representation.
New elements are placed by a selectable strategy,
with a fallback chain for the cases where the placement cannot be derived from the export.

Planned verbs:

- `merge` - graft an export into the base file, matching by `--id`;
  output to stdout, to a new file or in place
- `diff` - structural diff by id, added/removed/modified without reshuffling noise;
  an exit-code mode to use as a CI gate
- `format` - optional canonical formatter; merge never imposes it

## Scope

- Stable identifiers are required: matching is meaningful only if element ids survive regeneration.
  A generator that mints fresh UUIDs on every export is out of scope.
- A deliberate reorder is expressed by editing the base file.
  The committed order is the only carrier of curated order,
  so reordering on the export side counts as noise.
- The input is treated as a complete export: a missing element means deletion.

Not goals: a query language and arbitrary transformations (dobby is not jq),
and a three-way merge driver for git branches.
YAML support and a git clean filter come after the first release.

## Status

Early development, nothing released yet.
The first release goes to crates.io.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE), at your option.
