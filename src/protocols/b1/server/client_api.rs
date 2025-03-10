use axum::{extract::State, response::IntoResponse, Json};

use crate::common::{datastore::Datastore, ResponseError};

use super::B1Datastore;

pub async fn get_peer_count(
    State(datastore): State<B1Datastore>,
) -> Result<impl IntoResponse, ResponseError> {
    //let datastore: B1Datastore = datastore.into();
    //
    //let peer_count = datastore.peer_count().await?;
    let peer_count = 0u32;

    Ok(Json(peer_count))
}

pub async fn get_peer_count_stream() {}
