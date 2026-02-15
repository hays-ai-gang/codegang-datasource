use actix_web::{web, HttpResponse};

use crate::model::QueueContract;
use crate::storage::AppState;

/// GET /api/queue-contracts
pub async fn list(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(state.get_queue_contracts())
}

/// GET /api/queue-contracts/{topic}
pub async fn get(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    match state.get_queue_contract(&path.into_inner()) {
        Some(q) => HttpResponse::Ok().json(q),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Queue contract not found"})),
    }
}

/// POST /api/queue-contracts — insert or replace
pub async fn upsert(state: web::Data<AppState>, body: web::Json<QueueContract>) -> HttpResponse {
    state.upsert_queue_contract(body.into_inner());
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// DELETE /api/queue-contracts/{topic}
pub async fn delete(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    match state.delete_queue_contract(&path.into_inner()) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"status": "deleted"})),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({"error": e})),
    }
}
