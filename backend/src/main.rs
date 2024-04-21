use std::clone;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, guard};
use actix_web::web::{Data, route, service};

struct AppState {
    app_name: String,
    loading_count: Mutex<i32>,
}


#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    let mut value = data.loading_count.lock().unwrap();
    *value += 5;
    println!("{:?}", "|".repeat(*value as usize));
    format!("Hello {app_name}! Run {value}")
}

#[get("/go")]
async fn echo(req: HttpRequest) -> impl Responder {
    let url = req.url_for("foo", &["1", "2", "3"]); // <- generate URL for "foo" resource
    if let Ok(url) = url {
        format!("url: {:?}", url)
    } else {
        format!("Couldn't generate URL: {:?}", url)
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        app_name: String::from("Actx"),
        loading_count: Mutex::new(0),
    });
    let server_ip_address = IpAddr::V4([127, 0, 0, 1].into());
    let server_port: u16 = 8080;
    if let Ok(started_server) = start_server(state, server_ip_address, server_port).await {
        // TODO This would be reached if the server is stopped, not started
        println!("Started Server successfully at {}:{}", server_ip_address.to_string(), server_port);
        Ok(started_server)
    } else {
        println!("Failed to start server.");
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to start server"))
    }
}

async fn start_server(state: Data<AppState>, ip_addr: IpAddr, server_port: u16) -> std::io::Result<()> {
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(config_public)
            .configure(config_home)
    })
        .bind((ip_addr, server_port))?
        .run()
        .await?)
}

fn config_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/")
        .route(web::get().to(|| async {HttpResponse::Ok().body("Hey, hello! This is the public section of this server.")}))
    );
}

fn config_home(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("home")
            .guard(guard::Host("localhost"))
            .route("alarm", web::to(||async { HttpResponse::Ok().body("reached alarm section") }))
    );
}

