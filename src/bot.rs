use std::collections::HashMap;
use std::env;
use hyper::{Body, Request, Response, Client};
use hyper::client::HttpConnector;
use hyper::body::to_bytes;
use hyper_tls::HttpsConnector;
use std::{convert::Infallible};
use random_string::generate;
use crate::message::{IncomingMessage, OutgoingMessage};
use urlencoding::encode;

pub struct Bot {
    token: String,
    random_string: String,
    client: Client<HttpsConnector<HttpConnector>>
}

impl Bot {
    async fn execute_method(self: &Bot, method_name: &str, params: Option<HashMap<&str, &str>>) {
        let base = format!("https://api.telegram.org/bot{}/{}", self.token, method_name);

        let url = match params {
            Some(map) => map.into_iter().fold(format!("{}{}", base, "?"), |acc, (key, value)| {
                let ampersand = if &acc[acc.len() - 1..] == "?" {""} else {"&"};
                format!("{}{}{}={}", acc, ampersand, key, encode(value))
            }),
            None => base,
        };

        let result = self.client.get(url.parse().unwrap()).await;

        match result {
            Ok(response) => {
                if response.status().as_u16() > 399 {
                    panic!("Failed to set webhookUrl, http status code: {}", response.status());
                }
            },
            Err(error) => panic!("{}", error),
        }
    }

    pub fn new() -> Bot {
        let token = env::var("TOKEN").unwrap();
        let charset = "1234567890abcdef";
        let random_string = generate(6, charset);
        println!("{}", random_string);
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        Bot {token, random_string, client}
    }

    pub async fn handle_request(self: &Bot, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let uri = req.uri().to_string();
        
        
        if uri.replace("/", "") != self.random_string {
            Ok::<Response<Body>, Infallible>(Response::builder().status(404).body(Body::from("Not found")).unwrap())
        } else {
            let bytes = to_bytes(req.into_body()).await.unwrap();
            let incoming_message: IncomingMessage = serde_json::from_slice(&bytes).unwrap();
            let outgoing_message = OutgoingMessage::from_incoming(&incoming_message);
            let from_id = format!("{}", incoming_message.message.from.id);
            let mut params: HashMap<&str, &str> = HashMap::new();
            params.insert("chat_id", &from_id);
            params.insert("text", &outgoing_message.text);

            self.execute_method("sendMessage", Some(params)).await;
            
            Ok::<Response<Body>, Infallible>(Response::new(Body::from("OK")))
        }
    }

    pub async fn start(self: &Bot) {
        let mut params: HashMap<&str, &str> = HashMap::new();
        let webhook_host = env::var("WEBHOOK_HOST").unwrap();
        let url = format!("{}/{}", webhook_host, self.random_string);
        params.insert("url", &url);

        self.execute_method("setWebhook", Some(params)).await;
    }
}