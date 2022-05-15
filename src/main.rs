// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use std::sync::Mutex;
// use std::time::Duration;

// struct AppState {
//     app_name: String,
// }

// struct AppStateWithCounter {
//     counter: Mutex<i32>,
// }

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// async fn index() -> impl Responder {
//     "Hello world again"
// }

// async fn index_by_mutex_state(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap();
//     *counter += 1;

//     format!("Request number: {counter}")
// }

// #[get("/web")]
// async fn index_by_state(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name;
//     format!("Hello {app_name}")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });
//     HttpServer::new( move || {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//             .service(
//                 web::scope("/app")
//                     .route("/index.html", web::get().to(index))
//             )
//             .app_data(web::Data::new(AppState {
//                 app_name: String::from("Actix web"),
//             }))
//             .service(index_by_state)
//             .app_data(counter.clone())
//             .route("/count", web::get().to(index_by_mutex_state))
//     })
//     .keep_alive(Duration::from_secs(60))
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// async fn main() -> std::io::Result<()> {
//     // Note: web::Data created _outside_ HttpServer::new closure
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });

//     HttpServer::new(move || {
//         // move counter into the closure
//         App::new()
//             .app_data(counter.clone()) // <- register the created data
//             .route("/", web::get().to(index_by_mutex_state))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let auth_token = "";
    let url = "https://api.binance.com/api/v3/exchangeInfo?symbol=BNBBTC";
    // let resp = get_data_from_spotify("https://api.spotify.com/v1/search");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await;
    println!("{:?}", response)
}

// async fn get_data_from_spotify(url: &str) {
//     let client = reqwest::Client::new();
//     let response = client
//         .get(url)
//         // confirm the request using send()
//         .send()
//         .await
//         // the rest is the same!
//         .unwrap()
//         .text()
//         .await;
//     println!("{:?}", response)
// }
