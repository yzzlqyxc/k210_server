use std::{fs::File, io::{Cursor, Write}, time::Duration};

use axum::{extract::{self, multipart, Multipart, Path, State}, handler::HandlerWithoutStateExt, http::Method, middleware::AddExtension, response::{Html, Response}, routing::{get, post}, Extension, Router};
use image::{io::Reader, GenericImageView};
use serde_json::json;
use tokio::time::{sleep};
use tower_http::cors::{Any, CorsLayer};

use crate::{AsyncMap, AsyncSocket};

pub async fn https(mp : AsyncMap, socket: AsyncSocket) {
    let cors = CorsLayer::new()
                            .allow_headers(Any)
                            .allow_methods([Method::POST, Method::GET])
                            .allow_origin(Any);


    let app = Router::new()
                    .route("/getUserList", get(get_user_list))
                    .route("/getUserHistory/:id", get(get_user_history))
                    .route("/getPic", post({
                        let state = (mp.clone(), socket.clone());
                        move | body |  get_picture(body, state)
                    }))
                    .layer(cors)
                    .with_state((mp, socket));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub async fn get_picture(mut mult : Multipart, state: (AsyncMap, AsyncSocket)) -> String {
    let item = mult.next_field().await.unwrap().unwrap();
    let content = item.bytes().await.unwrap().to_vec();
    let mut pixels : Vec<u8> = Vec::new();

    let reader = Reader::new(Cursor::new(content)).with_guessed_format().unwrap();
    let image = reader.decode().unwrap();
    let height = image.height();
    let width = image.width();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let r = pixel[0] >> 3;
            let g = pixel[1] >> 2;
            let b = pixel[2] >> 3;
            let n_pixel = ((r as u16) << 11) | ((g as u16) << 5) | (b as u16);

            let low = (n_pixel & 255) as u8;
            let high = ((n_pixel >> 8) & 255) as u8;
            pixels.push(low);
            pixels.push(high);
        }
    }
    println!("{}, {}", height, width);
    println!("{}", pixels.len());
    let t : &[u8] = &pixels;
    println!("{}", t.len());

    let t = mult.next_field().await.unwrap().unwrap();
    let ip_port = String::from_utf8(t.bytes().await.unwrap().to_vec()).unwrap();

    let (mp, socket) = state;
    let t = mp.clone();
    let mut mp_clone = t.lock().unwrap();
    let addr = mp_clone.get_mut(&ip_port);
    if let Some(soc_addr) = addr {
        soc_addr.send_pic(&pixels, socket); 
        "Ok".to_owned()
    } else{
        println!("nothing");
        "Error".to_owned()
    }
}

pub async fn get_user_list(State(state): State<(AsyncMap, AsyncSocket)>) -> String{
    println!("in");
    let (mp, _) = state;
    let t = mp.clone();
    let mp_clone = t.lock().unwrap();
    let addrs : Vec<String> = mp_clone.keys().cloned().collect();
    // let addrs : Vec<String> = vec!["123.1.1.1".to_owned(), "12312".to_owned()];
    let response = json!({
        "addrs" : addrs
    });
    response.to_string()
}

pub async fn get_user_history(Path(id) : Path<String>, State(state): State<(AsyncMap, AsyncSocket)>) -> String{
    let (mp, _) = state;
    let t = mp.clone();
    let mp_clone = t.lock().unwrap();

    let t = if mp_clone.contains_key(&id) {
        mp_clone.get(&id).unwrap().get_history()
    } else {
        Vec::new()
    };
    let response = json!({
        "histories" : t
    });
    response.to_string()
}
