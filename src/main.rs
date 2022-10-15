use hyper::{Server};
use std::{convert::Infallible};
use hyper::service::{make_service_fn, service_fn};

pub mod bot;
pub mod tls;

use crate::bot::Bot;
use crate::tls::get_tls_listener;

#[tokio::main]
async fn main() {
    let bot: &'static Bot = Box::leak(Box::new(Bot::new()));
    bot.start().await;

    let make_svc = make_service_fn(|_conn| async move {
        Ok::<_, Infallible>(service_fn(move |req| bot.handle_request(req)))
    });

    let tls_listener = get_tls_listener();
    let server = Server::builder(tls_listener).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
