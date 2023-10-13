use actix_web::*;
use mysql as my;
use my::prelude::*;
use serde_derive::Deserialize;
use crate::db::db_pool;
use crate::model::professionals::{Professional};

// Função para obter uma página de profissionais
#[get("/professionals")]
pub async fn get_professionals(
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let pool = db_pool();
    let mut conn = pool.get_conn().expect("Failed to get database connection");

    // Parâmetros de paginação
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    // Calcular o índice inicial com base na página e itens por página
    let start_index = (page - 1) * per_page;

    // Consulta SQL para obter uma página de profissionais com LIMIT e OFFSET
    let query = format!("SELECT * FROM professionals LIMIT {} OFFSET {}", per_page, start_index);

    let professionals: Result<Vec<Professional>, my::Error> = conn
        .query_map(query, |(id, name, title, department)| {
            Professional {
                id,
                name,
                title,
                department,
            }
        });

    match professionals {
        Ok(professionals) => {
            HttpResponse::Ok().json(professionals)
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Failed to fetch professionals: {}", err))
        }
    }
}
#[derive(Deserialize)]
struct PaginationParams {
    page: Option<u32>,
    per_page: Option<u32>,
}
