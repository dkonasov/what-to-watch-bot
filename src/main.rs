use hyper::{Server};
use std::{convert::Infallible};
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};

pub mod bot;

use crate::bot::Bot;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let bot: &'static Bot = Box::leak(Box::new(Bot::new()));
    bot.start().await;

    let make_svc = make_service_fn(|_conn| async move {
        Ok::<_, Infallible>(service_fn(move |req| bot.handle_request(req)))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
