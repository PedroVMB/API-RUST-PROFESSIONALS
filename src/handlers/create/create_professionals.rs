use actix_web::{web, HttpResponse, Responder};
use mysql as my;
use my::prelude::*;
use actix_web::post;
use crate::db::db_pool;
use crate::model::professionals::{Professional};

// Função para criar a tabela se não existir
fn create_professionals_table_if_not_exists(conn: &mut my::PooledConn) -> Result<(), my::Error> {
    conn.query_drop("CREATE TABLE IF NOT EXISTS professionals (
        id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        title VARCHAR(255) NOT NULL,
        department VARCHAR(255) NOT NULL
    )")
}

#[post("/professionals")]
async fn create_professional(professional: web::Json<Professional>) -> impl Responder {
    let pool = db_pool();
    let mut conn = pool.get_conn().expect("Failed to get database connection");

    // Verifique se a tabela existe e crie-a se necessário
    if let Err(err) = create_professionals_table_if_not_exists(&mut conn) {
        return HttpResponse::InternalServerError().json(format!("Failed to create table: {}", err));
    }

    // Insira o registro do profissional
    if let Err(err) = conn.exec_drop(
        "INSERT INTO professionals (name, title, department) VALUES (?, ?, ?)",
        (
            &professional.name,
            &professional.title,
            &professional.department,
        ),
    ) {
        return HttpResponse::InternalServerError().json(format!("Failed to create professional: {}", err));
    }

    let last_insert_id = conn.last_insert_id();
    HttpResponse::Ok().json(format!("Professional with ID {} created.", last_insert_id))
}
