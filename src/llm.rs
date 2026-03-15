//! LLM Protocol Types
//!
//! Canonical Rust types matching the capdag media spec schemas for LLM operations.
//! These types are the single source of truth - both machfab and cartridges use them.
//!
//! Media specs:
//! - media:llm-generation-request;json;record
//! - media:llm-text-stream;ndjson;streaming
//! - media:llm-vocab-response;json;record
//! - media:llm-model-info;json;record
//!
//! Caps:
//! - cap:op=llm_inference - Text generation
//! - cap:op=llm_inference_constrained - Constrained generation with LLGuidance
//! - cap:op=llm_vocab - Vocabulary extraction
//! - cap:op=llm_model_info - Model info query

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// =============================================================================
// Request Type (optional discriminator)
// =============================================================================

/// Type of LLM request - used when a single command handles multiple operations
///
/// Prefer using different caps for different operations when possible.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    /// Standard text generation
    Generate,
    /// Get model vocabulary (for guidance initialization)
    GetVocab,
    /// Get model information
    GetInfo,
}

impl Default for RequestType {
    fn default() -> Self {
        Self::Generate
    }
}

// =============================================================================
// media:llm-generation-request;json;record
// =============================================================================

/// LLM Generation Request - input for all LLM caps
///
/// Matches: media:llm-generation-request;json;record
///
/// Note: The `request_type` field exists in the media spec for cases where a single
/// command handles multiple operations. Prefer using different caps for different
/// operations when possible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmGenerationRequest {
    /// The input text prompt for generation (required)
    pub prompt: String,

    /// Model specification - HuggingFace repo, local path, or alias (required)
    pub model_spec: String,

    /// Type of request (optional - prefer using different caps)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<RequestType>,

    /// System prompt for chat models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,

    /// Unique request identifier for correlation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,

    /// Sampling temperature (0.0-2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Top-k sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// Top-p (nucleus) sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Min-p sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>,

    /// Random seed for reproducibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Grammar constraint in GBNF format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grammar: Option<String>,

    /// JSON schema constraint for structured output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<JsonValue>,

    /// Constraint specification (alternative to grammar/json_schema)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint: Option<ConstraintSpec>,

    /// Override chat template (Jinja2 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_template: Option<String>,

    /// Sequences that stop generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// Maximum context length for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_context_length: Option<u32>,

    /// Batch size for processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,

    /// RoPE frequency base (for extended context models)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rope_freq_base: Option<f32>,

    /// RoPE frequency scale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rope_freq_scale: Option<f32>,
}

impl LlmGenerationRequest {
    /// Create a minimal generation request with just prompt and model
    pub fn new(prompt: impl Into<String>, model_spec: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            model_spec: model_spec.into(),
            request_type: None,
            system_prompt: None,
            request_id: None,
            max_tokens: None,
            temperature: None,
            top_k: None,
            top_p: None,
            min_p: None,
            seed: None,
            grammar: None,
            json_schema: None,
            constraint: None,
            chat_template: None,
            stop_sequences: None,
            max_context_length: None,
            batch_size: None,
            rope_freq_base: None,
            rope_freq_scale: None,
        }
    }

    /// Create request with default generation parameters
    pub fn with_defaults(prompt: impl Into<String>, model_spec: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            model_spec: model_spec.into(),
            request_type: Some(RequestType::Generate),
            system_prompt: None,
            request_id: None,
            max_tokens: Some(512),
            temperature: Some(0.7),
            top_k: Some(40),
            top_p: Some(0.9),
            min_p: Some(0.05),
            seed: Some(42),
            grammar: None,
            json_schema: None,
            constraint: None,
            chat_template: None,
            stop_sequences: None,
            max_context_length: Some(4096),
            batch_size: Some(2048),
            rope_freq_base: Some(10000.0),
            rope_freq_scale: Some(1.0),
        }
    }

    /// Create a vocabulary extraction request
    pub fn vocab_request(model_spec: impl Into<String>) -> Self {
        Self {
            prompt: String::new(),
            model_spec: model_spec.into(),
            request_type: Some(RequestType::GetVocab),
            system_prompt: None,
            request_id: None,
            max_tokens: None,
            temperature: None,
            top_k: None,
            top_p: None,
            min_p: None,
            seed: None,
            grammar: None,
            json_schema: None,
            constraint: None,
            chat_template: None,
            stop_sequences: None,
            max_context_length: None,
            batch_size: None,
            rope_freq_base: None,
            rope_freq_scale: None,
        }
    }

    /// Create a model info request
    pub fn model_info_request(model_spec: impl Into<String>) -> Self {
        Self {
            prompt: String::new(),
            model_spec: model_spec.into(),
            request_type: Some(RequestType::GetInfo),
            system_prompt: None,
            request_id: None,
            max_tokens: None,
            temperature: None,
            top_k: None,
            top_p: None,
            min_p: None,
            seed: None,
            grammar: None,
            json_schema: None,
            constraint: None,
            chat_template: None,
            stop_sequences: None,
            max_context_length: None,
            batch_size: None,
            rope_freq_base: None,
            rope_freq_scale: None,
        }
    }

    /// Get the effective request type (defaults to Generate if not specified)
    pub fn effective_request_type(&self) -> RequestType {
        self.request_type.unwrap_or(RequestType::Generate)
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("LlmGenerationRequest serialization should never fail")
    }

    /// Builder method to set request_id
    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = Some(id.into());
        self
    }

    /// Builder method to set constraint
    pub fn with_constraint(mut self, constraint: ConstraintSpec) -> Self {
        self.constraint = Some(constraint);
        self
    }

    /// Builder method to set JSON schema constraint
    pub fn with_json_schema(mut self, schema: JsonValue) -> Self {
        self.json_schema = Some(schema);
        self
    }

    /// Builder method to set grammar constraint
    pub fn with_grammar(mut self, grammar: impl Into<String>) -> Self {
        self.grammar = Some(grammar.into());
        self
    }
}

/// Constraint specification for guided generation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConstraintSpec {
    /// JSON schema constraint
    JsonSchema {
        schema: JsonValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    /// Regular expression constraint
    Regex {
        pattern: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    /// Lark grammar constraint
    Grammar {
        grammar: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    /// Tool calling constraint
    ToolCall {
        tools: Vec<ToolDefinition>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

/// Tool definition for constrained tool calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    /// JSON Schema for tool parameters
    pub parameters: JsonValue,
}

// =============================================================================
// media:llm-text-stream;ndjson;streaming
// =============================================================================

/// LLM Text Stream Message - NDJSON streaming output
///
/// Matches: media:llm-text-stream;ndjson;streaming
/// Each line is one of these message types, serialized as JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LlmStreamMessage {
    /// A generated token
    Token { text: String },

    /// Status update during generation
    Status {
        operation: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        progress: Option<f32>,
    },

    /// Generation complete
    Complete {
        generated_text: String,
        tokens_generated: usize,
        prompt_tokens: usize,
        finish_reason: String,
        generation_time_ms: u64,
    },

    /// Tool request detected during generation
    ToolRequest {
        partial_text: String,
        tool_name: String,
        tool_args: JsonValue,
    },

    /// Error during processing
    Error { code: String, message: String },
}

impl LlmStreamMessage {
    /// Create a token message
    pub fn token(text: impl Into<String>) -> Self {
        Self::Token { text: text.into() }
    }

    /// Create a status message
    pub fn status(
        operation: impl Into<String>,
        details: Option<String>,
        progress: Option<f32>,
    ) -> Self {
        Self::Status {
            operation: operation.into(),
            details,
            progress,
        }
    }

    /// Create a completion message
    pub fn complete(
        generated_text: impl Into<String>,
        tokens_generated: usize,
        prompt_tokens: usize,
        finish_reason: impl Into<String>,
        generation_time_ms: u64,
    ) -> Self {
        Self::Complete {
            generated_text: generated_text.into(),
            tokens_generated,
            prompt_tokens,
            finish_reason: finish_reason.into(),
            generation_time_ms,
        }
    }

    /// Create a tool request message
    pub fn tool_request(
        partial_text: impl Into<String>,
        tool_name: impl Into<String>,
        tool_args: JsonValue,
    ) -> Self {
        Self::ToolRequest {
            partial_text: partial_text.into(),
            tool_name: tool_name.into(),
            tool_args,
        }
    }

    /// Create an error message
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Error {
            code: code.into(),
            message: message.into(),
        }
    }

    /// Serialize to NDJSON line (no trailing newline)
    pub fn to_line(&self) -> String {
        serde_json::to_string(self).expect("LlmStreamMessage serialization should never fail")
    }

    /// Parse from NDJSON line
    pub fn from_line(line: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(line)
    }
}

/// Common finish reasons as constants
pub mod finish_reason {
    /// Reached end-of-sequence token
    pub const STOP: &str = "stop";
    /// Reached maximum token limit
    pub const LENGTH: &str = "length";
    /// Tool call detected, pausing for execution
    pub const TOOL_CALL: &str = "tool_call";
    /// Generation was interrupted
    pub const INTERRUPTED: &str = "interrupted";
    /// Constraint completed (for guided generation)
    pub const CONSTRAINT_SATISFIED: &str = "constraint_satisfied";
}

// =============================================================================
// media:llm-vocab-response;json;record
// =============================================================================

/// LLM Vocabulary Response
///
/// Matches: media:llm-vocab-response;json;record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmVocabResponse {
    /// Array of vocabulary tokens
    pub vocab: Vec<String>,

    /// Total vocabulary size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocab_size: Option<usize>,
}

impl LlmVocabResponse {
    /// Create a vocabulary response
    pub fn new(vocab: Vec<String>) -> Self {
        let vocab_size = vocab.len();
        Self {
            vocab,
            vocab_size: Some(vocab_size),
        }
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("LlmVocabResponse serialization should never fail")
    }
}

// =============================================================================
// media:llm-model-info;json;record
// =============================================================================

/// LLM Model Info Response
///
/// Matches: media:llm-model-info;json;record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmModelInfo {
    /// Model specification identifier
    pub model_spec: String,

    /// Size of the model's vocabulary
    pub vocab_size: usize,

    /// Maximum context length the model supports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<usize>,

    /// Dimension of embedding vectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_dim: Option<usize>,

    /// Model file size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_bytes: Option<u64>,

    /// Number of attention heads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_count: Option<usize>,

    /// Number of transformer layers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_count: Option<usize>,

    /// Whether the model has a chat template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_chat: Option<bool>,

    /// Whether the model supports tool use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_tools: Option<bool>,
}

impl LlmModelInfo {
    /// Create minimal model info
    pub fn new(model_spec: impl Into<String>, vocab_size: usize) -> Self {
        Self {
            model_spec: model_spec.into(),
            vocab_size,
            context_length: None,
            embedding_dim: None,
            file_size_bytes: None,
            head_count: None,
            layer_count: None,
            supports_chat: None,
            supports_tools: None,
        }
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("LlmModelInfo serialization should never fail")
    }
}

// =============================================================================
// Media URNs - canonical definitions (single source of truth)
// =============================================================================

/// Media URN for LLM generation request payload (JSON object)
pub const MEDIA_LLM_GENERATION_REQUEST: &str = "media:llm-generation-request;json;record";

/// Media URN for LLM streaming text output (NDJSON stream)
pub const MEDIA_LLM_TEXT_STREAM: &str = "media:llm-text-stream;ndjson;streaming";

/// Media URN for LLM vocabulary response (JSON object)
pub const MEDIA_LLM_VOCAB_RESPONSE: &str = "media:llm-vocab-response;json;record";

/// Media URN for LLM model info response (JSON object)
pub const MEDIA_LLM_MODEL_INFO_RESPONSE: &str = "media:llm-model-info;json;record";

// =============================================================================
// Cap URNs - canonical definitions
// =============================================================================

/// Cap URN for LLM text generation
pub const CAP_LLM_INFERENCE: &str = "cap:op=llm_inference;llm;ml-model;gguf;in=\"media:llm-generation-request;json;record\";out=\"media:llm-text-stream;ndjson;streaming\"";

/// Cap URN for constrained LLM generation with LLGuidance
pub const CAP_LLM_INFERENCE_CONSTRAINED: &str = "cap:op=llm_inference_constrained;constrained;llm;ml-model;gguf;in=\"media:llm-generation-request;json;record\";out=\"media:llm-text-stream;ndjson;streaming\"";

/// Cap URN for vocabulary extraction
pub const CAP_LLM_VOCAB: &str = "cap:op=llm_vocab;llm;ml-model;gguf;in=\"media:llm-generation-request;json;record\";out=\"media:llm-vocab-response;json;record\"";

/// Cap URN for model info query
pub const CAP_LLM_MODEL_INFO: &str = "cap:op=llm_model_info;llm;ml-model;gguf;in=\"media:llm-generation-request;json;record\";out=\"media:llm-model-info;json;record\"";

/// Cap URN for text embeddings generation
pub const CAP_GENERATE_EMBEDDINGS: &str = "cap:op=generate_embeddings;ml-model;gguf;in=\"media:textable\";out=\"media:embedding-vector;record;textable\"";

/// Cap URN for embedding dimensions query
pub const CAP_EMBEDDINGS_DIMENSIONS: &str = "cap:gguf;in=\"media:embeddings;gguf;model-spec;textable\";ml-model;op=embeddings_dimensions;out=\"media:integer;model-dim;numeric;textable\"";

/// Cap URN for vision/image description
pub const CAP_DESCRIBE_IMAGE: &str = "cap:gguf;in=\"media:image;png\";ml-model;op=describe_image;out=\"media:image-description;textable\";vision";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_request_serialization() {
        let req = LlmGenerationRequest::with_defaults("Hello", "model/test")
            .with_request_id("req-123");

        let json = req.to_json();
        let parsed = LlmGenerationRequest::from_json(&json).unwrap();

        assert_eq!(parsed.prompt, "Hello");
        assert_eq!(parsed.model_spec, "model/test");
        assert_eq!(parsed.request_id, Some("req-123".to_string()));
        assert_eq!(parsed.max_tokens, Some(512));
    }

    #[test]
    fn test_stream_message_token() {
        let msg = LlmStreamMessage::token("Hello");
        let line = msg.to_line();

        assert!(line.contains("\"type\":\"token\""));
        assert!(line.contains("\"text\":\"Hello\""));

        let parsed = LlmStreamMessage::from_line(&line).unwrap();
        if let LlmStreamMessage::Token { text } = parsed {
            assert_eq!(text, "Hello");
        } else {
            panic!("Expected Token");
        }
    }

    #[test]
    fn test_stream_message_complete() {
        let msg = LlmStreamMessage::complete("Generated text", 10, 5, finish_reason::STOP, 100);
        let line = msg.to_line();

        assert!(line.contains("\"type\":\"complete\""));
        assert!(line.contains("\"finish_reason\":\"stop\""));

        let parsed = LlmStreamMessage::from_line(&line).unwrap();
        if let LlmStreamMessage::Complete {
            tokens_generated,
            finish_reason,
            ..
        } = parsed
        {
            assert_eq!(tokens_generated, 10);
            assert_eq!(finish_reason, "stop");
        } else {
            panic!("Expected Complete");
        }
    }

    #[test]
    fn test_stream_message_error() {
        let msg = LlmStreamMessage::error("MODEL_NOT_FOUND", "Model not available");
        let line = msg.to_line();

        let parsed = LlmStreamMessage::from_line(&line).unwrap();
        if let LlmStreamMessage::Error { code, message } = parsed {
            assert_eq!(code, "MODEL_NOT_FOUND");
            assert_eq!(message, "Model not available");
        } else {
            panic!("Expected Error");
        }
    }

    #[test]
    fn test_vocab_response() {
        let resp = LlmVocabResponse::new(vec!["a".into(), "b".into(), "c".into()]);
        let json = resp.to_json();
        let parsed = LlmVocabResponse::from_json(&json).unwrap();

        assert_eq!(parsed.vocab.len(), 3);
        assert_eq!(parsed.vocab_size, Some(3));
    }

    #[test]
    fn test_model_info() {
        let info = LlmModelInfo::new("test-model", 32000);
        let json = info.to_json();
        let parsed = LlmModelInfo::from_json(&json).unwrap();

        assert_eq!(parsed.model_spec, "test-model");
        assert_eq!(parsed.vocab_size, 32000);
    }

    #[test]
    fn test_constraint_spec_json_schema() {
        let constraint = ConstraintSpec::JsonSchema {
            schema: serde_json::json!({"type": "object"}),
            description: Some("Test".into()),
        };
        let json = serde_json::to_string(&constraint).unwrap();
        assert!(json.contains("\"type\":\"json_schema\""));
    }

    #[test]
    fn test_constraint_spec_regex() {
        let constraint = ConstraintSpec::Regex {
            pattern: r"\d+".into(),
            description: None,
        };
        let json = serde_json::to_string(&constraint).unwrap();
        assert!(json.contains("\"type\":\"regex\""));
    }
}
