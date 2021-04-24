use std::env::{self, args};

use actix_web::{App, Either, HttpResponse, HttpServer, Responder, web, middleware::Logger};
use dotenv::dotenv;
use actix_files::NamedFile;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args:Vec<String> = args().collect();
    let mut port = if let Some( s) =  args.get(1){
        s.parse::<u16>().unwrap()
    }else{
        env::var("port")
            .map(|s|s.parse::<u16>().unwrap_or(8080))
            .unwrap_or(8080)
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    loop {
        let bind = HttpServer::new(|| {
            App::new()
                // .wrap(Logger::default())
                .wrap(Logger::new("\"%r\" %s %{User-Agent}i"))
                .route("/{filename:.*}", web::get().to(index))
            }).bind(format!("localhost:{}",port));
        if let Ok(s) = bind{
            return s.run().await;
        }
        log::log!(log::Level::Warn,"Port {} had been used. will change port",{port});
        port = port.overflowing_add(1).0;
    }
}

async fn index(web::Path(filename): web::Path<String>) -> Either<NamedFile, impl Responder> {
    let root = option_env!("root").unwrap_or("");
    let root: std::path::PathBuf = std::path::PathBuf::from(root);
    let file = root.join(filename);
    let file = if file.is_file(){
        file
    }else if file.is_dir(){
        let index = file.join("index.html");
        if index.is_file(){
            index
        }else{
            root.join("index.html")
        }
    }else{
        root.join("index.html")
    };
    if let Ok(file) = NamedFile::open(file) {
        Either::A(file)
    } else {
        Either::B(HttpResponse::NotFound())
    }
}