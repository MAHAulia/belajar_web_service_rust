use std::{env, sync::Arc};
use axum::{
    body::Body, extract::{Extension, Path}, http::StatusCode, response::{IntoResponse, Response}, Json
};
use futures::StreamExt;
use mongodb::{bson::{doc, to_bson, Bson, Document, Uuid}, Client, Collection};
use serde_json::to_string;
use crate::models::{User, CreateUser};

// Handler for get all user data
pub async fn get_all_user(Extension(arc_client): Extension<Arc<Client>>)-> impl IntoResponse {
    let db = arc_client.database(env::var("DB_NAME").unwrap_or_default().as_str());
    let collection:Collection<Document> = db.collection("users");
    let filter = doc! {};
    let cursor  = collection.find(filter, None).await.expect("Failed to query data");

    // Create a vector to store the documents
    let datacollections: Vec<Result<Document, mongodb::error::Error>> = cursor.collect().await;

    let mut documents:Vec<User> = Vec::new();

    for result in datacollections {
        match result {
            Ok(document) => {
                // Process the document here
                let userdata: User = mongodb::bson::from_document(document).expect("Failed to convert");
                
                documents.push(userdata);
            }
            Err(e) => {
                // Handle error
                eprintln!("Error retrieving document: {}", e);
            }
        }
    }
    // Serialize the vector of documents to JSON
    let json_data = serde_json::to_string(&documents).unwrap();

    // Return a response with the JSON data
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(json_data))
        .unwrap()

}

// Handler for get detail of user
pub async fn get_user(Extension(arc_client): Extension<Arc<Client>>,Path(id): Path<String>) -> impl IntoResponse{
    let db = arc_client.database(env::var("DB_NAME").unwrap_or_default().as_str());
    let collection:Collection<Document> = db.collection("users");
    let uuid = Uuid::parse_str(id).expect("Failed to parse UUID string");
    let filter = doc! { "id": uuid };
    let result = collection.find_one(filter, None).await.unwrap();
    if let Some(doc) = result {
        let user: User = mongodb::bson::from_document(doc).expect("Failed to convert data");
        Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::from(to_string(&user).unwrap()))
        .unwrap()
    } else {
        Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("User not found"))
        .unwrap()
    }
}

// Handler for add new user from json
pub async fn add_user(Extension(arc_client): Extension<Arc<Client>>,Json(payload): Json<CreateUser>) -> impl IntoResponse{
    let mycollection:Collection<Document> = arc_client.database(env::var("DB_NAME").unwrap_or_default().as_str()).collection("users");
    let user = User {
        id: Uuid::new(),
        name: payload.username,
        email: payload.email,
    };
    let bson_value = match to_bson(&user){
        Ok(bson) => bson,
        Err(_e) => return (StatusCode::UNPROCESSABLE_ENTITY, Json(User::default())),
    };
    let doc = match bson_value {
        Bson::Document(doc) => doc,
        _ => return (StatusCode::EXPECTATION_FAILED, Json(User::default())),
    };
    match mycollection.insert_one(doc, None).await {
        Ok(_) => (StatusCode::CREATED, Json(user)),
        Err(_e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(User::default())),
    }
}

// Handler for update user
pub async fn update_user(Extension(arc_client): Extension<Arc<Client>>, Path(id): Path<String>, Json(payload): Json<CreateUser>) -> impl IntoResponse{
    let db = arc_client.database(env::var("DB_NAME").unwrap_or_default().as_str());
    let collection:Collection<Document> = db.collection("users");
    let uuid = Uuid::parse_str(id).expect("Failed to parse UUID string");
    let filter = doc! { "id": uuid };
    let update = doc! { "$set": doc! {"name": payload.username, "email": payload.email} };
    let _updatedata = collection.update_one(filter, update, None).await.unwrap();

    let filter2 = doc! { "id": uuid };
    let result = collection.find_one(filter2, None).await.unwrap();
    if let Some(doc) = result {
        let user: User = mongodb::bson::from_document(doc).expect("Failed to convert data");
        Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::from(to_string(&user).unwrap()))
        .unwrap()
    } else {
        Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("User not found"))
        .unwrap()
    }
}
// Define a handler that performs an operation and may return an error
pub async fn delete_user(Extension(arc_client): Extension<Arc<Client>>, Path(id): Path<String>) -> impl IntoResponse {
    let db = arc_client.database(env::var("DB_NAME").unwrap_or_default().as_str());
    let collection:Collection<Document> = db.collection("users");
    let uuid = Uuid::parse_str(id).expect("Failed to parse UUID string");
    let filter = doc! { "id": uuid };
    let result = collection.delete_one(filter, None).await.unwrap();

    
    if result.deleted_count > 0 {
        Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::from("User berhasil dihapus"))
        .unwrap()
    } else {
        Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("User not found"))
        .unwrap()
    }
}

