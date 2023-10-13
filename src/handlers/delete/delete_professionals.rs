use actix_web::*;
use mysql as my;
use my::prelude::*;
use crate::db::db_pool;

#[delete("/professionals/{id}")]
pub async fn delete_professional(path: web::Path<(u32,)>) -> HttpResponse {
    let pool = db_pool();
    let mut conn = pool.get_conn().expect("Failed to get database connection");

    // Consulta SQL para excluir o profissional
    let query = "DELETE FROM professionals WHERE id = ?";

    if let Err(err) = conn.exec_drop(query, (path.0,)) {
        return HttpResponse::InternalServerError().json(format!("Failed to delete professional: {}", err));
    }

    HttpResponse::Ok().json("Professional deleted successfully")
}
