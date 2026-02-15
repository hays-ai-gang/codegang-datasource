use actix_web::{web, HttpResponse};

use crate::model::ServiceDefinition;
use crate::storage::AppState;

/// GET /api/services
pub async fn list(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(state.get_services())
}

/// GET /api/services/{name}
pub async fn get(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    match state.get_service(&path.into_inner()) {
        Some(s) => HttpResponse::Ok().json(s),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Service not found"})),
    }
}

/// POST /api/services — insert or replace
pub async fn upsert(state: web::Data<AppState>, body: web::Json<ServiceDefinition>) -> HttpResponse {
    state.upsert_service(body.into_inner());
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// DELETE /api/services/{name}
pub async fn delete(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    match state.delete_service(&path.into_inner()) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"status": "deleted"})),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({"error": e})),
    }
}
