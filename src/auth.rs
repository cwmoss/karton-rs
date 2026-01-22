use crate::AppState;
use crate::store;

use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
};
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct User {
    pub album: String,
    pub is_admin: bool,
}

#[derive(Default, Deserialize, Serialize)]
struct Counter(usize);

pub async fn check_auth_middleware(
    State(app_state): State<Arc<AppState>>,
    Path(params): Path<HashMap<String, String>>,
    session: Session,
    req: Request,
    next: Next,
) -> Response {
    // requires the http crate to get the header name
    // if req.headers().get(CONTENT_TYPE).unwrap() != "application/json" {
    //     return Err(StatusCode::BAD_REQUEST);
    // }

    //Ok(next.run(req).await)
    // println!("check-auth");
    // println!("check auth {:?} {}", app_state.anon, album);
    let album = "??";

    let counter: Counter = session.get("_counter").await.unwrap().unwrap_or_default();
    let mut user: User = session.get("_user").await.unwrap().unwrap_or_default();

    if counter.0 == 7 {
        user.is_admin = true;
    }

    session.insert("_counter", counter.0 + 1).await.unwrap();

    eprintln!("Current count: {} user: {:?}", counter.0, user);

    session.insert("_user", user).await.unwrap();

    eprintln!(
        "check auth {:?} {} {:?}",
        app_state.anon,
        req.uri(),
        params.get("album")
    );

    let response = next.run(req).await;
    response
}

pub fn get_or_create_admin_secret(store: store::Store) -> String {
    match store.get_admin_secret() {
        Some(secret) => secret,
        None => store.save_admin_secret(gen_secret(13)),
    }
}

fn gen_secret(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    rand::rng().fill_bytes(&mut buf);
    let b64 = general_purpose::STANDARD.encode(&buf);
    b64.trim_end_matches('=')
        .replace('+', "-")
        .replace('/', "_")
}

struct MyStruct {
    var: PathBuf,
}

impl MyStruct {
    fn secret(&self, name: &str) -> String {
        let s = gen_secret(64);
        let mut path = self.var.clone();
        path.push(name);
        path.push(format!(".secret.{}", s));
        fs::File::create(path).unwrap();
        // println!("\n\nlogin: /karton/login/{}/{}\n\n", name, s);
        s
    }
}
