use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::sync::Arc;
use std::env;

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

#[derive(Serialize, Deserialize)]
struct SearchResult {
    hits: Vec<Hit>,
}

#[derive(Serialize, Deserialize)]
struct Hit {
    id: String,
    title: String,
    date: String,
    description: String,
    reading_time: String,
    slug: String,
}

#[derive(Serialize)]
struct SearchRequest {
    q: String,
}

#[derive(Clone)]
struct MeiliConfig {
    url: String,
    api_key: String,
}

#[get("/search")]
async fn search_handler(
    query: web::Query<SearchQuery>,
    client: web::Data<Arc<Client>>,
    config: web::Data<MeiliConfig>,
) -> impl Responder {
    if query.q.trim().is_empty() {
        return HttpResponse::BadRequest().body("Query cannot be empty");
    }

    let url = format!("{}/indexes/posts/search", config.url);
    
    let request_body = SearchRequest {
        q: query.q.trim().to_string(),
    };

    let response = match client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&request_body)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Request error: {}", e);
            return HttpResponse::InternalServerError().body("Search server error");
        }
    };

    match response.json::<SearchResult>().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprintln!("Failed to parse response: {}", e);
            HttpResponse::InternalServerError().body("Invalid response format")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();

    let config = MeiliConfig {
        url: env_var("MEILI_URL").unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        }),
        api_key: env_var("MEILI_API_KEY").unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        }),
    };

    let client = Arc::new(Client::new());

    println!("Server starting on localhost:8080");
    println!("Connected to Meilisearch at {}", config.url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(search_handler)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

fn env_var(name: &str) -> Result<String, String> {
    env::var(name).map_err(|e| format!("Environment variable {} not set: {}", name, e))
}
