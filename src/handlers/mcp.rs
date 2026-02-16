use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_lab::sse;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};

use crate::storage::AppState;

// ── Session registry ─────────────────────────────────────────────

pub type Sessions = Arc<RwLock<HashMap<String, mpsc::Sender<sse::Event>>>>;

pub fn new_sessions() -> Sessions {
    Arc::new(RwLock::new(HashMap::new()))
}

// ── JSON-RPC types ───────────────────────────────────────────────

#[derive(Deserialize)]
#[allow(dead_code)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    #[serde(default)]
    params: serde_json::Value,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Serialize)]
struct JsonRpcError {
    code: i64,
    message: String,
}

impl JsonRpcResponse {
    fn success(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: serde_json::Value, code: i64, message: String) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: None,
            error: Some(JsonRpcError { code, message }),
        }
    }
}

// ── GET /sse ─────────────────────────────────────────────────────

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct SseQuery {
    pub session_id: Option<String>,
}

pub async fn sse_handler(
    req: HttpRequest,
    sessions: web::Data<Sessions>,
) -> impl Responder {
    let session_id = uuid::Uuid::new_v4().to_string();
    let (tx, mut rx) = mpsc::channel::<sse::Event>(32);

    sessions.write().await.insert(session_id.clone(), tx);

    let host = req
        .headers()
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost:8080");

    let scheme = if host.contains("localhost") || host.starts_with("127.") {
        "http"
    } else {
        "http"
    };

    let endpoint_url = format!("{scheme}://{host}/message?session_id={session_id}");

    let session_id_clone = session_id.clone();
    let sessions_clone = sessions.into_inner().clone();

    let stream = async_stream::stream! {
        // First event: tell client where to POST messages
        yield Ok::<_, std::convert::Infallible>(sse::Event::Data(
            sse::Data::new(endpoint_url)
                .event("endpoint")
        ));

        // Stream responses
        loop {
            match tokio::time::timeout(Duration::from_secs(30), rx.recv()).await {
                Ok(Some(event)) => yield Ok::<_, std::convert::Infallible>(event),
                Ok(None) => break, // channel closed
                Err(_) => {
                    // Send keepalive comment
                    yield Ok::<_, std::convert::Infallible>(sse::Event::Comment("keepalive".into()));
                }
            }
        }

        // Cleanup session
        sessions_clone.write().await.remove(&session_id_clone);
    };

    sse::Sse::from_stream(stream)
        .with_keep_alive(Duration::from_secs(15))
}

// ── POST /message ────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct MessageQuery {
    session_id: String,
}

pub async fn message_handler(
    query: web::Query<MessageQuery>,
    body: web::Json<serde_json::Value>,
    state: web::Data<AppState>,
    sessions: web::Data<Sessions>,
) -> HttpResponse {
    let session_id = &query.session_id;

    let sessions_read = sessions.read().await;
    let tx = match sessions_read.get(session_id) {
        Some(tx) => tx.clone(),
        None => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Session not found"
            }));
        }
    };
    drop(sessions_read);

    let req: JsonRpcRequest = match serde_json::from_value(body.into_inner()) {
        Ok(r) => r,
        Err(e) => {
            let err = JsonRpcResponse::error(
                serde_json::Value::Null,
                -32700,
                format!("Parse error: {e}"),
            );
            let _ = tx
                .send(sse::Event::Data(
                    sse::Data::new(serde_json::to_string(&err).unwrap())
                        .event("message"),
                ))
                .await;
            return HttpResponse::Accepted().finish();
        }
    };

    // Notifications have no id and expect no response
    if req.id.is_none() {
        return HttpResponse::Accepted().finish();
    }

    let id = req.id.unwrap();
    let response = handle_method(&req.method, &req.params, &state, id.clone());

    let json = serde_json::to_string(&response).unwrap();
    let _ = tx
        .send(sse::Event::Data(sse::Data::new(json).event("message")))
        .await;

    HttpResponse::Accepted().finish()
}

// ── Method dispatch ──────────────────────────────────────────────

fn handle_method(
    method: &str,
    params: &serde_json::Value,
    state: &AppState,
    id: serde_json::Value,
) -> JsonRpcResponse {
    match method {
        "initialize" => handle_initialize(id),
        "tools/list" => handle_tools_list(id),
        "tools/call" => handle_tools_call(params, state, id),
        _ => JsonRpcResponse::error(id, -32601, format!("Method not found: {method}")),
    }
}

fn handle_initialize(id: serde_json::Value) -> JsonRpcResponse {
    JsonRpcResponse::success(
        id,
        serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "codegang-datasource",
                "version": "0.1.0"
            }
        }),
    )
}

fn handle_tools_list(id: serde_json::Value) -> JsonRpcResponse {
    JsonRpcResponse::success(
        id,
        serde_json::json!({
            "tools": [
                {
                    "name": "get_datasource",
                    "description": "Get the full datasource including all services, queue contracts, nosql contracts, and proto contracts.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {},
                        "required": []
                    }
                },
                {
                    "name": "list_services",
                    "description": "List all microservices with their gRPC servers/clients, queue bindings, and metadata.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {},
                        "required": []
                    }
                },
                {
                    "name": "get_service",
                    "description": "Get a single microservice definition by name.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Service name, e.g. 'contest-engine-grpc'"
                            }
                        },
                        "required": ["name"]
                    }
                },
                {
                    "name": "list_queue_contracts",
                    "description": "List all service-bus queue/topic contracts with their message schemas.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {},
                        "required": []
                    }
                },
                {
                    "name": "get_queue_contract",
                    "description": "Get a single queue/topic contract by topic name, including its message schema.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "topic": {
                                "type": "string",
                                "description": "Topic name, e.g. 'user-registered'"
                            }
                        },
                        "required": ["topic"]
                    }
                },
                {
                    "name": "list_nosql_contracts",
                    "description": "List all NoSQL entity contracts with their table names and schemas.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {},
                        "required": []
                    }
                },
                {
                    "name": "get_nosql_contract",
                    "description": "Get a single NoSQL entity contract by entity name, including its schema.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "entity": {
                                "type": "string",
                                "description": "Entity name, e.g. 'BrokerContestSettings'"
                            }
                        },
                        "required": ["entity"]
                    }
                },
                {
                    "name": "list_proto_contracts",
                    "description": "List all protobuf/gRPC contracts with their raw .proto definitions.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {},
                        "required": []
                    }
                },
                {
                    "name": "get_proto_contract",
                    "description": "Get a single protobuf/gRPC contract by name, including the raw .proto file text.",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Proto contract name, e.g. 'UsersGrpcService'"
                            }
                        },
                        "required": ["name"]
                    }
                }
            ]
        }),
    )
}

fn handle_tools_call(
    params: &serde_json::Value,
    state: &AppState,
    id: serde_json::Value,
) -> JsonRpcResponse {
    let tool_name = params
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or(serde_json::json!({}));

    let result = match tool_name {
        "get_datasource" => {
            let ds = state.get_datasource();
            serde_json::to_string_pretty(&ds).unwrap()
        }
        "list_services" => {
            let svcs = state.get_services();
            serde_json::to_string_pretty(&svcs).unwrap()
        }
        "get_service" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("");
            match state.get_service(name) {
                Some(s) => serde_json::to_string_pretty(&s).unwrap(),
                None => format!("Service '{name}' not found"),
            }
        }
        "list_queue_contracts" => {
            let qcs = state.get_queue_contracts();
            serde_json::to_string_pretty(&qcs).unwrap()
        }
        "get_queue_contract" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap_or("");
            match state.get_queue_contract(topic) {
                Some(q) => serde_json::to_string_pretty(&q).unwrap(),
                None => format!("Queue contract '{topic}' not found"),
            }
        }
        "list_nosql_contracts" => {
            let ncs = state.get_nosql_contracts();
            serde_json::to_string_pretty(&ncs).unwrap()
        }
        "get_nosql_contract" => {
            let entity = args.get("entity").and_then(|v| v.as_str()).unwrap_or("");
            match state.get_nosql_contract(entity) {
                Some(n) => serde_json::to_string_pretty(&n).unwrap(),
                None => format!("NoSQL contract '{entity}' not found"),
            }
        }
        "list_proto_contracts" => {
            let pcs = state.get_proto_contracts();
            serde_json::to_string_pretty(&pcs).unwrap()
        }
        "get_proto_contract" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("");
            match state.get_proto_contract(name) {
                Some(p) => serde_json::to_string_pretty(&p).unwrap(),
                None => format!("Proto contract '{name}' not found"),
            }
        }
        _ => {
            return JsonRpcResponse::error(
                id,
                -32602,
                format!("Unknown tool: {tool_name}"),
            );
        }
    };

    JsonRpcResponse::success(
        id,
        serde_json::json!({
            "content": [
                {
                    "type": "text",
                    "text": result
                }
            ]
        }),
    )
}
