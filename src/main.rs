use actix_web::*;
use handlers::create::create_professionals::create_professional;
use handlers::get::get_professionals::get_professionals;
use handlers::update::update_professionals::update_professional;
use handlers::delete::delete_professionals::delete_professional;

mod db;
mod handlers;
mod model;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
            .service(create_professional)
            .service(get_professionals)
            .service(update_professional)
            .service(delete_professional)

    });
    let porta: i32 = 9091;
    let api = api.bind(format!("127.0.0.1:{}", porta))
        .expect("Não conseguiu conectar...");

    println!("Conectado com sucesso! \n  http://localhost:{}", porta);

    api.run()
        .await

}
