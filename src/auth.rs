use crate::AppState;
use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
};
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn check_auth_middleware(
    State(app_state): State<Arc<AppState>>,
    Path(params): Path<HashMap<String, String>>,
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

    eprintln!(
        "check auth {:?} {} {:?}",
        app_state.anon,
        req.uri(),
        params.get("album")
    );

    let response = next.run(req).await;
    response
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
