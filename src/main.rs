use std::fs;
use axum::{routing::{get, post}, Router, Json};
use axum::response::{Html, IntoResponse};
use bendy::decoding::{Decoder, Error, Object};
use serde::{Serialize,Deserialize};
use axum::extract::Multipart;

#[derive(Serialize, Debug, Deserialize)]
struct Torrent{
    info: Info,
    announce: String,
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date")]
    creation_date: Option<u64>,
    comment: Option<String>,
    #[serde(rename = "created by")]
    created_by: Option<String>,
    encoding: Option<String>,
}

#[derive(Serialize, Debug, Deserialize)]
struct Info{
    name: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/get_torrent", post(get_torrent));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {

    Html(include_str!("../index.html"))

}

async fn get_torrent(mut multipart:Multipart) -> impl IntoResponse {

    let mut torrent_file:Torrent = Torrent {
        info: Info { name: "".to_string() },
        announce: "".to_string(),
        announce_list: None,
        creation_date: None,
        comment: None,
        created_by: None,
        encoding: None
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        if name == "file" {
            let bytes = field.bytes().await.unwrap();
            // let torrent_file: Torrent = serde_bencode::from_bytes(bytes.as_ref()).unwrap();
            torrent_file = serde_bencoded::from_bytes(bytes.as_ref()).unwrap();
            // torrent.info.name = torrent_file.info.name;
        }
    }

    Json(torrent_file)
}

