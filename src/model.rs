use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Top-level datasource containing services and all contract registries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datasource {
    #[serde(default)]
    pub services: Vec<ServiceDefinition>,
    #[serde(default)]
    pub queue_contracts: Vec<QueueContract>,
    #[serde(default)]
    pub nosql_contracts: Vec<NosqlContract>,
    #[serde(default)]
    pub proto_contracts: Vec<ProtoContract>,
}

// ── Services ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_clients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<ServiceQueueConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_http_server: Option<bool>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceQueueConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_queues: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_queues: Option<Vec<String>>,
}

// ── Queue / Service-Bus contracts ────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueContract {
    /// Unique topic/queue name, e.g. "user-registered".
    pub topic_name: String,
    /// Human+AI readable purpose of this topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The message schema carried on this topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_schema: Option<MessageSchema>,
}

/// Language-agnostic description of a message/event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSchema {
    /// Schema name, e.g. "UserRegisteredEvent".
    pub name: String,
    /// Ordered list of fields in this message.
    pub fields: Vec<SchemaField>,
    /// Free-form notes for AI agents (edge cases, invariants, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// A single field inside a MessageSchema or NosqlSchema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    /// Type expressed language-agnostically: string, i64, f64, bool,
    /// uuid, datetime, bytes, enum(A|B|C), optional<T>, repeated<T>, map<K,V>.
    pub field_type: String,
    /// What this field means — written for AI consumption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ── NoSQL contracts ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NosqlContract {
    /// Unique entity/table identifier, e.g. "BrokerContestSettings".
    pub entity_name: String,
    /// The underlying table/collection name in the DB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// Human+AI readable purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The entity schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<MessageSchema>,
}

// ── Proto / gRPC contracts ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoContract {
    /// Proto service/file name, e.g. "UsersGrpcService".
    pub name: String,
    /// Raw .proto file content.
    pub raw_proto: String,
}
