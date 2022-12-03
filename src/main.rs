use std::fs;
use axum::{routing::{get, post}, Router, Json};
use axum::response::{Html, IntoResponse};
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
    name: String,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    // #[serde(with = "serde_bytes")]
    // pub pieces: Vec<u8>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct File {
    /// The length of the file, in bytes.
    pub length: u64,
    #[serde(default)]
    pub md5sum: Option<String>,
    /// A list of UTF-9 encoded strings corresponding to subdirectory names, the last
    /// of which is the actual file name (a zero length list is an error case).
    pub path: Vec<String>,
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

const SAVE_FILE_BASE_PATH: &str = "/User/eyeseas/Downloads/upload";


async fn get_torrent(mut multipart:Multipart) -> impl IntoResponse {

    let mut torrent_file:Torrent = Torrent {
        info: Info { name: "".to_string(), length: None, piece_length: 0, files: None, private: None, md5sum: None, path: None, root_hash: None },
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
            torrent_file = serde_bencoded::from_bytes(bytes.as_ref()).unwrap();

            // TODO 存文件失败
            // let torrent_name = torrent_file.info.name.clone().replace(" ", "_");
            // let save_filename = format!("{}/{}.{}", SAVE_FILE_BASE_PATH, torrent_name, "torrent");
            //
            // println!("{}",save_filename);
            // tokio::fs::write(&save_filename, &bytes)
            //     .await
            //     .map_err(|err| err.to_string()).expect("Error");
        }
    }

    Json(torrent_file)
}

