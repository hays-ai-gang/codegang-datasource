use actix_web::{web, HttpResponse};

use crate::model::Datasource;
use crate::storage::AppState;

/// GET /api/datasource
pub async fn get(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(state.get_datasource())
}

/// PUT /api/datasource
pub async fn replace(state: web::Data<AppState>, body: web::Json<Datasource>) -> HttpResponse {
    state.replace_datasource(body.into_inner());
    HttpResponse::Ok().json(state.get_datasource())
}
