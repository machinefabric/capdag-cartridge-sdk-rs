# MachineFabric cartridge SDK for Rust

This public Rust library provides shared types and policies used by MachineFabric cartridges. It re-exports CapDAG and adds product-level data contracts that must stay identical across cartridge implementations.

## Add the SDK

The workspace pins this crate and CapDAG together. In an external cartridge, use the released crate version required by that cartridge's target MachineFabric release.

```rust
use machfab_cartridge_sdk::llm::LlmGenerationRequest;

let request = LlmGenerationRequest::with_defaults(
    "Summarize this document",
    "hf:Qwen/Qwen2.5-0.5B-Instruct",
);
```

## Module reference

| Module | Contract |
| --- | --- |
| `llm` | LLM request, stream, vocabulary, model-information types, media URNs, cap URNs, and model-backend classification. |
| `prompt` | Prompt-strategy classification and the shared default system prompt. |
| `pages` | One-based page/index selection with ordered, deduplicated, clamped ranges. |
| `net_retry` | Shared transient-network classification and bounded retry policy. |
| `structured_queries` | Structured-query definitions, rendering, registration, and decision-result types. |

The Rust API documentation beside each exported item is authoritative for fields, defaults, failure behavior, and examples. Language-neutral routing and runtime behavior belongs to the [CapDAG specification](../../capdag/docs/01-overview.md).

## Verify changes

From this package directory, run:

```bash
cargo test
```

When a shared wire type or prompt decision changes, update the Go and Swift/Objective-C mirrors and their same-numbered substantive tests in the same change.
