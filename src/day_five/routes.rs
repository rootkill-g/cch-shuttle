use axum::{extract::Query, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParameters {
    offset: Option<usize>,
    limit: Option<usize>,
}

pub async fn msg_for_mr_grinch(
    pagination: Query<QueryParameters>,
    Json(names): Json<Vec<String>>,
) -> impl IntoResponse {
    tracing::info!("day_5 => POST RECEIVED: {:?}, {:?}", pagination, names);

    let offset = pagination.offset.unwrap_or(0);

    let list_slice = match pagination.limit {
        Some(l) => {
            let limit = names.len().min(offset + l);
            &names[offset..limit]
        }
        None => &names[offset..],
    };

    Json(serde_json::json!(list_slice))
}
