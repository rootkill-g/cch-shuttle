use axum::{extract::Query, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use serde_json as sj;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParameters {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

pub async fn msg_for_mr_grinch(
    pagination: Query<QueryParameters>,
    Json(names): Json<Vec<String>>,
) -> impl IntoResponse {
    tracing::info!("day_5 => POST RECEIVED: {:?}, {:?}", pagination, names);

    let offset = pagination.offset.unwrap_or(0);

    let mut list_slice = match pagination.limit {
        Some(l) => {
            let limit = names.len().min(offset + l);
            &names[offset..limit]
        }
        None => &names[offset..],
    };

    if list_slice.is_empty() {
        return Json(sj::json!(list_slice));
    }

    let res = match pagination.split {
        Some(s) => {
            if s == 0 {
                sj::json!(list_slice)
            } else {
                let mut result: Vec<Vec<String>> = Vec::new();

                loop {
                    if list_slice.len() <= s {
                        result.push(list_slice.to_owned());
                        break;
                    }

                    let (l, r) = list_slice.split_at(s);
                    result.push(l.to_owned());
                    if r.is_empty() {
                        break;
                    }
                    list_slice = r;
                }
                sj::json!(result)
            }
        }
        None => sj::json!(list_slice),
    };

    Json(res)
}
