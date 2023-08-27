use base64::{engine::general_purpose, Engine as _};
use flowsnet_platform_sdk::logger;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use webhook_flows::{request_received, send_response};
//base64 encoder decoder

#[derive(Debug, Deserialize)]
struct MethodNMsg {
    message: String,
    method: u8,
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| handler(headers, qry, body)).await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, _qry: HashMap<String, Value>, body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let msg_meth: MethodNMsg = match serde_json::from_slice(&body) {
        Ok(data) => data,
        Err(e) => {
            log::error!("JSON Parse Error {}", e);
            send_response(
                400,
                vec![(String::from("content-type"), String::from("text/plain"))],
                "Bad Request".as_bytes().to_vec(),
            );
            return;
        }
    };

    match msg_meth.method {
        1 => {
            let encoded = general_purpose::STANDARD_NO_PAD.encode(msg_meth.message);
            send_response(
                200,
                vec![(String::from("content-type"), String::from("text/plain"))],
                encoded.as_bytes().to_vec(),
            );
        }
        //todo errorhandling
        0 => {
            let decoded = general_purpose::STANDARD_NO_PAD
                .decode(msg_meth.message)
                .unwrap();
            send_response(
                200,
                vec![(String::from("content-type"), String::from("text/plain"))],
                decoded,
            );
        }
        _ => {
            send_response(
                400,
                vec![(String::from("content-type"), String::from("text/plain"))],
                "Invalid method".as_bytes().to_vec(),
            );
        }
    }
}
