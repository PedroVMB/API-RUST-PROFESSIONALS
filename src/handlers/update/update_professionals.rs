use actix_web::*;
use mysql as my;
use my::prelude::*;
use crate::db::db_pool;
use crate::model::professionals::{Professional};

// Função para atualizar um profissional
#[put("/professionals/{id}")]
pub async fn update_professional(
    path: web::Path<(u32,)>,
    professional: web::Json<Professional>,
) -> HttpResponse {
    let pool = db_pool();
    let mut conn = pool.get_conn().expect("Failed to get database connection");

    // Consulta SQL para atualizar o profissional
    let query = "UPDATE professionals SET name = ?, title = ?, department = ? WHERE id = ?";

    if let Err(err) = conn.exec_drop(
        query,
        (
            &professional.name,
            &professional.title,
            &professional.department,
            path.0,
        ),
    ) {
        return HttpResponse::InternalServerError().json(format!("Failed to update professional: {}", err));
    }

    HttpResponse::Ok().json("Professional updated successfully")
}
