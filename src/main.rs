// This sample was written with some inspiration from https://github.com/actix/examples/tree/master/async_ex1
//
// This example web app does the following for a given `id`:
// 1. fetch and parse UserData from `/users/:id`
// 2. fetch and parse UserPosts from `/users/:id/posts`
// 3. merge those together in a combined response.

#[macro_use]
extern crate actix_web;

use std::io;

use actix_web::{
    App,
    client::Client,
    Error, HttpResponse, HttpServer, web::{self, BytesMut},
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct UserData {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PostData {
    id: u32,
    title: String,
    body: String,
}

#[derive(Debug, Serialize)]
struct MyResponse {
    user: UserData,
    posts: Vec<PostData>,
}

async fn get_user_by_id(client: &Client, id: u32) -> Result<UserData, Error> {
    let mut res = client
        .get(format!("http://127.0.0.1:3000/users/{}", id))
        .send()
        .await
        .map_err(Error::from)?;

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }

    let user_data: UserData = serde_json::from_slice(&body).unwrap();
    Ok(user_data)
}

async fn get_posts_by_id(client: &Client, id: u32) -> Result<Vec<PostData>, Error> {
    let mut res = client
        .get(format!("http://127.0.0.1:3000/users/{}/posts", id))
        .send()
        .await
        .map_err(Error::from)?;

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }

    let posts_for_user: Vec<PostData> = serde_json::from_slice(&body).unwrap();
    Ok(posts_for_user)
}

// generator for a route
#[get("/{id}")]
async fn backend(client: web::Data<Client>, info: web::Path<u32>) -> Result<HttpResponse, Error> {
    // extract id from path
    let id = info.into_inner();

    // call backend services
    let u = get_user_by_id(&client, id).await?;
    let p = get_posts_by_id(&client, id).await?;

    // merge into a response
    let response = MyResponse { user: u, posts: p };

    // serialize the response
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap()))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=warn");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // attach a HTTP client to the App
            .data(Client::default())
            // register a HTTP service/route
            .service(backend)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}