use actix_web::{web, HttpResponse};

use crate::model::ProtoContract;
use crate::storage::AppState;

/// GET /api/proto-contracts
pub async fn list(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(state.get_proto_contracts())
}

/// GET /api/proto-contracts/{name}
pub async fn get(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    match state.get_proto_contract(&path.into_inner()) {
        Some(p) => HttpResponse::Ok().json(p),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Proto contract not found"})),
    }
}

/// POST /api/proto-contracts — insert or replace
pub async fn upsert(state: web::Data<AppState>, body: web::Json<ProtoContract>) -> HttpResponse {
    state.upsert_proto_contract(body.into_inner());
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// DELETE /api/proto-contracts/{name}
pub async fn delete(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    match state.delete_proto_contract(&path.into_inner()) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"status": "deleted"})),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({"error": e})),
    }
}
