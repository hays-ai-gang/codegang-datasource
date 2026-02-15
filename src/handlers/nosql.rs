use actix_web::{web, HttpResponse};

use crate::model::NosqlContract;
use crate::storage::AppState;

/// GET /api/nosql-contracts
pub async fn list(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(state.get_nosql_contracts())
}

/// GET /api/nosql-contracts/{entity}
pub async fn get(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    match state.get_nosql_contract(&path.into_inner()) {
        Some(n) => HttpResponse::Ok().json(n),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "NoSQL contract not found"})),
    }
}

/// POST /api/nosql-contracts — insert or replace
pub async fn upsert(state: web::Data<AppState>, body: web::Json<NosqlContract>) -> HttpResponse {
    state.upsert_nosql_contract(body.into_inner());
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// DELETE /api/nosql-contracts/{entity}
pub async fn delete(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    match state.delete_nosql_contract(&path.into_inner()) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"status": "deleted"})),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({"error": e})),
    }
}
