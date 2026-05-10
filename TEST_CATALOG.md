# Rust Test Catalog

**Total Tests:** 16

**Numbered Tests:** 6

**Unnumbered Tests:** 10

**Numbered Tests Missing Descriptions:** 0

**Numbering Mismatches:** 0

**⚠ Duplicate test numbers detected: 1 number(s) used more than once.**
Unique numbered tests are listed first. Duplicate-number entries are grouped after them and marked with ⚠. Unnumbered tests are listed in their own group.

This catalog lists all tests in the Rust codebase.

| Test # | Function Name | Description | File |
|--------|---------------|-------------|------|
| | | | |
| test999 ⚠ | `test999_absent_template_yields_raw` | / **Core regression guard.** The empty `chat_template` field / means a base / completion model — the cartridge must NOT / chat-template the input. Routing here is what makes a / well-formed instruct model degrade to raw completion was the / bug; routing the OTHER way (raw model into chat-template / rendering) would be the equivalent regression — the rendered / `<\|im_start\|>` etc. would tokenize as plain text and corrupt / the completion. | src/prompt.rs:212 |
| test999 ⚠ | `test999_default_system_prompt_is_task_agnostic` | / `DEFAULT_SYSTEM_PROMPT` is generic enough to work for any / input. Pin this constant so a future change that tightens it / (e.g. inserts task-specific instructions) is a deliberate / commit, not an accidental one. | src/prompt.rs:274 |
| test999 ⚠ | `test999_jinja_template_yields_chat_templated` | / Jinja-template models route through `ChatTemplated`. Forgetting / this collapses to `Raw`, which feeds the rendered chat / scaffolding to the tokenizer as plain text — produces / degenerate output where the model treats `<\|im_start\|>` as / arbitrary characters instead of a special token. | src/prompt.rs:168 |
| test999 ⚠ | `test999_short_name_template_yields_chat_templated` | / `chat-template-short` (model identifies its template by / registered short-name) must also route through chat / templating — the cartridge will resolve the short name via / its backend's template registry. | src/prompt.rs:188 |
| test999 ⚠ | `test999_unknown_chat_template_value_yields_raw` | / An unknown `chat_template` value falls into `Raw` — we don't / invent a chat-template behaviour for a tag we don't know. / Future template tags must be classified explicitly here / rather than silently routed through `ChatTemplated` (which / might call into a backend code path that can't handle them). | src/prompt.rs:257 |
| test999 ⚠ | `test999_whitespace_only_system_prompt_dropped_for_chat_templated` | / Whitespace-only system prompt is dropped. Some templates / emit a `<\|im_start\|>system\n\n<\|im_end\|>` envelope around the / empty body — wasted tokens and a confused turn structure. / `Some("")` and `Some("   ")` both collapse to `None`. | src/prompt.rs:231 |
| | | | |
| unnumbered | `test_backend_for_model_spec_candle` |  | src/llm.rs:542 |
| unnumbered | `test_backend_for_model_spec_gguf` |  | src/llm.rs:516 |
| unnumbered | `test_backend_for_model_spec_mlx` |  | src/llm.rs:529 |
| unnumbered | `test_constraint_spec_tags` |  | src/llm.rs:499 |
| unnumbered | `test_generation_request_round_trip` | Round-trip: a generation request serializes and deserializes to equivalent content. | src/llm.rs:406 |
| unnumbered | `test_model_info_round_trip` |  | src/llm.rs:477 |
| unnumbered | `test_stream_message_complete_round_trip` |  | src/llm.rs:432 |
| unnumbered | `test_stream_message_error_round_trip` |  | src/llm.rs:451 |
| unnumbered | `test_stream_message_token_round_trip` | Round-trip: each stream message variant serializes and deserializes to itself. | src/llm.rs:419 |
| unnumbered | `test_vocab_response_round_trip` |  | src/llm.rs:466 |

---

## ⚠ Duplicate Test Numbers

The following test numbers are assigned to more than one function. Keep the first occurrence at the existing number and renumber the rest using the suggested free numbers below.

### test999 (6 occurrences)

- `test999_absent_template_yields_raw` — src/prompt.rs:212
- `test999_default_system_prompt_is_task_agnostic` — src/prompt.rs:274
- `test999_jinja_template_yields_chat_templated` — src/prompt.rs:168
- `test999_short_name_template_yields_chat_templated` — src/prompt.rs:188
- `test999_unknown_chat_template_value_yields_raw` — src/prompt.rs:257
- `test999_whitespace_only_system_prompt_dropped_for_chat_templated` — src/prompt.rs:231

**Suggested free number(s):** test1000, test998, test1001, test997, test1002

---

## Unnumbered Tests

The following tests are cataloged but do not currently participate in numeric test indexing.

- `test_backend_for_model_spec_candle` — src/llm.rs:542
- `test_backend_for_model_spec_gguf` — src/llm.rs:516
- `test_backend_for_model_spec_mlx` — src/llm.rs:529
- `test_constraint_spec_tags` — src/llm.rs:499
- `test_generation_request_round_trip` — src/llm.rs:406
- `test_model_info_round_trip` — src/llm.rs:477
- `test_stream_message_complete_round_trip` — src/llm.rs:432
- `test_stream_message_error_round_trip` — src/llm.rs:451
- `test_stream_message_token_round_trip` — src/llm.rs:419
- `test_vocab_response_round_trip` — src/llm.rs:466

---

*Generated from Rust source tree*
*Total tests: 16*
*Total numbered tests: 6*
*Total unnumbered tests: 10*
*Total numbered tests missing descriptions: 0*
*Total numbering mismatches: 0*
