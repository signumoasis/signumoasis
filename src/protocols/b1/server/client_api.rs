use axum::{extract::State, response::IntoResponse, Json};

use crate::common::ResponseError;

use super::B1Datastore;

pub async fn get_peer_count(
    State(datastore): State<B1Datastore>,
) -> Result<impl IntoResponse, ResponseError> {
    let peer_count = datastore.peer_count().await?;

    Ok(Json(peer_count))
}

pub async fn get_peer_count_stream() {}
