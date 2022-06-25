use std::convert::Infallible;
use std::env::args;
use std::io::{Error, ErrorKind};
use std::path::Path;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

const BASE: &str = "/app/html/";

async fn read_file(uri: &str) -> Result<String, Error> {
    println!("request for {}", uri);
    let mut path = String::new();
    let mut base = BASE;
    let a : Vec<String> = args().collect();
    if a.len() > 1 {
        base = &a[1];
        path.push_str(&a[1]);
        path.push_str("/");
    } else {
        path.push_str(BASE);
    }
    if uri == "/" {
        path.push_str("index.html");
    } else {
        path.push_str(uri);
    }
    println!("path: {}", path);
    if let Ok(p) = Path::new(&path).canonicalize() {
        println!("canonicalized: {:?}", p);
        if p.exists() && p.starts_with(&base) {
            return tokio::fs::read_to_string(path).await;
        }
    }
    Err(Error::from(ErrorKind::NotFound))
}

async fn server(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("request: {} {}", request.method(), request.uri());
    if let Ok(response) = read_file(request.uri().path()).await {
        return Ok(Response::new(Body::from(response)));
    }
    let notfound = read_file("404.html").await.unwrap(); // this should exist
    Ok(Response::builder()
        .status(404)
        .body(Body::from(notfound))
        .unwrap())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(server)) });

    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
