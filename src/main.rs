use axum::{
   /*  body::{Body,Bytes}, */
    extract::{RawPathParams/* ,Path,State */},
    routing::{get,/* put */},
    response::{IntoResponse},
    Router,
};

mod store;

const PATH: &str = "/home/sambo/projects/raft_server/src/data/database2";

#[axum_macros::debug_handler]
async fn kv_get(
   key: RawPathParams
    // State(state): State<store::KeyValueStoreHandler>,
) ->  impl IntoResponse {
    let mut vec = Vec::new();
    for (endpoint, key) in &key {     
       let pairs = loop_on_two_key_value_params(&endpoint,&key);
       vec.push(pairs)
    }
    let mut val = vec[0].values();
    let second_value = val.next().unwrap().to_string();
    let val_bytes = second_value.as_bytes(); 
    let db = store::KeyValueStoreHandler::new(&PATH);
 
      if let Some(value) = db.get(&val_bytes/* .as_bytes */).expect("something bad happened") {
        value
    } else {
        Vec::<u8>::from("ERROR FROM kv_get")
    } 
}

async fn handle_write_data(key: &[u8], value: &[u8]) {
        // println!("key {:?} = value {:?}",key,value);
        store::KeyValueStoreHandler::put(&PATH,key, value)
            .expect("Unable to write key-value pair to DB");      
}

fn loop_on_two_key_value_params<'a>(first: &'a str, second: &'a str)-> std::collections::HashMap<&'a str, &'a str> {
    let mut result = std::collections::HashMap::new();
    result.insert(first, second);
    result
}

async fn kv_set(params: RawPathParams) {
    let mut vec = Vec::new();
    for (endpoint, key) in &params {     
       let pairs = loop_on_two_key_value_params(&endpoint,&key);
       vec.push(pairs)
    }
    let mut key = vec[0].values();
    let mut val = vec[1].values();
    let first_value = key.next().unwrap().to_string();
    let second_value = val.next().unwrap().to_string();
    let key_bytes = first_value.as_bytes();
    let val_bytes = second_value.as_bytes();

    println!("key {:?} val {:?}",first_value,second_value);
    handle_write_data(key_bytes,val_bytes).await
 }


 
#[tokio::main]
async fn main() {
   axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap(); 


}

fn app() -> Router {
    Router::new()
        .route("/get/:whatever", get(kv_get))
        .route("/:key/:value", get(kv_set))
}
