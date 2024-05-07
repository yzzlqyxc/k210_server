use std::time::Duration;

use axum::{extract::{Path, State}, handler::HandlerWithoutStateExt, response::Html, routing::get, Extension, Router};
use tokio::time::{sleep};

use crate::{AsyncMap, AsyncSocket};

pub async fn https(mp : AsyncMap, socket: AsyncSocket) {
    // build our application with a route
    let app = Router::new()
                    .route("/getUserList", get(get_user_list))
                    .route("/getUserHistory/:id", get(get_user_history))
                    .with_state((mp, socket));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub async fn get_user_list(State(state): State<(AsyncMap, AsyncSocket)>) -> String {
    let (mp, _) = state;
    let t = mp.clone();
    let mp_clone = t.lock().unwrap();
    let addrs : Vec<String> = mp_clone.keys().cloned().collect();
    serde_json::to_string(&addrs).unwrap()
}

pub async fn get_user_history(Path(id) : Path<String>, State(state): State<(AsyncMap, AsyncSocket)>) -> String{
    let (mp, _) = state;
    let t = mp.clone();
    let mp_clone = t.lock().unwrap();

    if mp_clone.contains_key(&id) {
        let t  = mp_clone.get(&id).unwrap().get_history();
        serde_json::to_string(&t).unwrap()
    } else {
        "None".to_owned()        
    }
}