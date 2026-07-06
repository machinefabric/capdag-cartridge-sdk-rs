# Rust Test Catalog

**Total Tests:** 36

**Numbered Tests:** 36

**Unnumbered Tests:** 0

**Numbered Tests Missing Descriptions:** 0

**Numbering Mismatches:** 0

**⚠ Duplicate test numbers detected: 1 number(s) used more than once.**
Unique numbered tests are listed first. Duplicate-number entries are grouped after them and marked with ⚠. Unnumbered tests are listed in their own group.

This catalog lists all tests in the Rust codebase.

| Test # | Function Name | Description | File |
|--------|---------------|-------------|------|
| test0001 | `test0001_generation_request_round_trip` | Round-trip: a generation request serializes and deserializes to equivalent content. | src/llm.rs:409 |
| test0002 | `test0002_stream_message_token_round_trip` | Round-trip: each stream message variant serializes and deserializes to itself. | src/llm.rs:422 |
| test0003 | `test0003_stream_message_complete_round_trip` | TEST0003: Stream message complete round trip | src/llm.rs:436 |
| test0004 | `test0004_stream_message_error_round_trip` | TEST0004: Stream message error round trip | src/llm.rs:456 |
| test0005 | `test0005_vocab_response_round_trip` | TEST0005: Vocab response round trip | src/llm.rs:472 |
| test0006 | `test0006_model_info_round_trip` | TEST0006: Model info round trip | src/llm.rs:484 |
| test0007 | `test0007_constraint_spec_tags` | TEST0007: Constraint spec tags | src/llm.rs:507 |
| test0008 | `test0008_backend_for_model_spec_gguf` | TEST0008: Backend for model spec gguf | src/llm.rs:525 |
| test0009 | `test0009_backend_for_model_spec_mlx` | TEST0009: Backend for model spec mlx | src/llm.rs:539 |
| test0010 | `test0010_backend_for_model_spec_candle` | TEST0010: Backend for model spec candle | src/llm.rs:553 |
| test0011 | `test0011_jinja_template_yields_chat_templated` | / Jinja-template models route through `ChatTemplated`. Forgetting / this collapses to `Raw`, which feeds the rendered chat / scaffolding to the tokenizer as plain text — produces / degenerate output where the model treats `<\|im_start\|>` as / arbitrary characters instead of a special token. | src/prompt.rs:168 |
| test0012 | `test0012_short_name_template_yields_chat_templated` | / `chat-template-short` (model identifies its template by / registered short-name) must also route through chat / templating — the cartridge will resolve the short name via / its backend's template registry. | src/prompt.rs:188 |
| test0013 | `test0013_absent_template_yields_raw` | / **Core regression guard.** The empty `chat_template` field / means a base / completion model — the cartridge must NOT / chat-template the input. Routing here is what makes a / well-formed instruct model degrade to raw completion was the / bug; routing the OTHER way (raw model into chat-template / rendering) would be the equivalent regression — the rendered / `<\|im_start\|>` etc. would tokenize as plain text and corrupt / the completion. | src/prompt.rs:212 |
| test0014 | `test0014_whitespace_only_system_prompt_dropped_for_chat_templated` | / Whitespace-only system prompt is dropped. Some templates / emit a `<\|im_start\|>system\n\n<\|im_end\|>` envelope around the / empty body — wasted tokens and a confused turn structure. / `Some("")` and `Some("   ")` both collapse to `None`. | src/prompt.rs:231 |
| test0015 | `test0015_unknown_chat_template_value_yields_raw` | / An unknown `chat_template` value falls into `Raw` — we don't / invent a chat-template behaviour for a tag we don't know. / Future template tags must be classified explicitly here / rather than silently routed through `ChatTemplated` (which / might call into a backend code path that can't handle them). | src/prompt.rs:257 |
| test0016 | `test0016_default_system_prompt_is_task_agnostic` | / `DEFAULT_SYSTEM_PROMPT` is generic enough to work for any / input. Pin this constant so a future change that tightens it / (e.g. inserts task-specific instructions) is a deliberate / commit, not an accidental one. | src/prompt.rs:274 |
| test0060 | `test0060_index_range_grammar` | TEST0060: full grammar — singles, ranges, open ranges, comma lists, written order preserved, duplicates dropped on first occurrence. | src/pages.rs:123 |
| test0061 | `test0061_index_range_clamps_past_end` | TEST0061: over-long ranges clamp to the document instead of erroring (the old pdf parser hard-errored on `1-100` of a 10-page doc). | src/pages.rs:143 |
| test0062 | `test0062_index_range_hard_errors` | TEST0062: genuinely impossible selections stay hard errors with actionable messages. | src/pages.rs:156 |
| test0229 | `test0229_structured_query_creation` | / Test basic StructuredQuery creation with name, description, prompt template, and schema / Validates that all fields are properly initialized and accessible | src/structured_queries/mod.rs:474 |
| test0230 | `test0230_prompt_generation` | / Test Tera template rendering with variable substitution / Validates that prompt templates render correctly with provided substitutions | src/structured_queries/mod.rs:518 |
| test0232 | `test0232_query_builder` | / Test StructuredQueryBuilder pattern for fluent query creation / Validates that builder pattern works correctly with method chaining and metadata | src/structured_queries/mod.rs:562 |
| test0233 | `test0233_output_validation` | / Test JSON schema validation against query outputs / Validates that LLM outputs are properly validated against expected schemas | src/structured_queries/mod.rs:579 |
| test0234 | `test0234_make_decision_query_type` | / Test specific make_decision query loading and validation / Validates that binary choice queries work correctly with template rendering | src/structured_queries/mod.rs:611 |
| test0235 | `test0235_dynamic_schema_generation` | / Test Tera template rendering for dynamic schema generation / Validates that schemas can be generated dynamically with template variables | src/structured_queries/mod.rs:686 |
| test0236 | `test0236_static_schema_fallback` | / Test fallback to static schema when no template exists / Validates that queries without schema templates use their static schemas | src/structured_queries/mod.rs:763 |
| test0237 | `test0237_builder_with_schema_template` | / Test builder pattern with dynamic schema templates / Validates that builder can create queries with both prompt and schema templates | src/structured_queries/mod.rs:790 |
| test8200 | `test8200_transient_status_then_success_retries_and_succeeds` | TEST8200: a transient 503 followed by a 200 retries exactly once and the caller receives the 200 — the core "ride out a blip" contract. | src/net_retry.rs:349 |
| test8201 | `test8201_permanent_status_is_not_retried` | TEST8201: a permanent 404 is returned to the caller on the FIRST attempt with NO retry. Retrying a 404 would mask a genuine "does not exist" and waste time; this pins that 404 is terminal. | src/net_retry.rs:363 |
| test8202 | `test8202_exhausted_retries_surface_the_last_transient_response` | TEST8202: when every attempt is a transient status, the retry budget is spent and the LAST response (the real 503) is returned verbatim — the failure is exposed, not swallowed into a fabricated success/empty. | src/net_retry.rs:381 |
| test8203 | `test8203_transport_failure_is_retried_then_surfaced` | TEST8203: a transport-level failure (connect refused — no server) is transient, so it is retried up to the budget, and the final transport error is returned (not a panic, not a swallowed Ok). | src/net_retry.rs:403 |
| test8204 | `test8204_single_attempt_policy_does_not_retry` | TEST8204: max_attempts == 1 disables retrying — a transient 503 is returned immediately after a single attempt. Pins that the policy knob actually gates the loop. | src/net_retry.rs:421 |
| test8205 | `test8205_backoff_is_exponential_and_capped` | TEST8205: backoff grows exponentially and is capped at max_delay. Pure arithmetic on the policy — no clock — so it is deterministic. | src/net_retry.rs:438 |
| test8206 | `test8206_jitter_is_bounded` | TEST8206: jitter stays within [0, delay] and 0 maps to 0. Guards the de-correlation invariant — a jittered wait must never exceed the base. | src/net_retry.rs:457 |
| | | | |
| test0231 ⚠ | `test0231_prose_schemas_bound_their_text_fields` | / Every prose-output query bounds its free-text string field with a `maxLength`, / so grammar-constrained generation always closes the JSON object instead of being / truncated mid-string when the token budget is reached. This is the schema-side / half of the summarize/ask truncation fix — without the bound the model can run a / field to the token cap, leaving an unterminated JSON that fails to parse. | src/structured_queries/mod.rs:499 |
| test0231 ⚠ | `test0231_registry_operations` | / Test StructuredQueryRegistry loading and query retrieval operations / Validates that registry loads queries from embedded files and provides access | src/structured_queries/mod.rs:543 |

---

## ⚠ Duplicate Test Numbers

The following test numbers are assigned to more than one function. Keep the first occurrence at the existing number and renumber the rest using the suggested free numbers below.

### test0231 (2 occurrences)

- `test0231_prose_schemas_bound_their_text_fields` — src/structured_queries/mod.rs:499
- `test0231_registry_operations` — src/structured_queries/mod.rs:543

**Suggested free number(s):** test228

---

*Generated from Rust source tree*
*Total tests: 36*
*Total numbered tests: 36*
*Total unnumbered tests: 0*
*Total numbered tests missing descriptions: 0*
*Total numbering mismatches: 0*
