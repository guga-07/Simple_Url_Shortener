use crate::AppState;
use crate::models::{CreateLink, StoredLink};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use rand::RngExt;
use sqlx;

fn generate_code() -> String {
    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// previous implementation of  Handler create_line Implementation Via Hashmap
// pub async fn create_link(
//     State(state): State<AppState>,
//     Json(body): Json<CreateLink>,
// ) -> (StatusCode, Json<StoredLink>) {
//     let code = generate_code();
//     let link = StoredLink {
//         original_url: body.original_url,
//         short_code: code.clone(),
//         clicks: 0,
//     };
//     let mut map = state.lock().unwrap();
//     map.insert(code, link.clone());
//     (StatusCode::CREATED, Json(link))
// }

pub async fn create_link(
    State(state): State<AppState>,
    Json(body): Json<CreateLink>,
) -> (StatusCode, Json<StoredLink>) {
    let code = generate_code();
    sqlx::query("INSERT INTO links (original_url, short_code, clicks) VALUES (?, ?, ?)")
        .bind(&body.original_url)
        .bind(&code)
        .bind(0i32)
        .execute(&state)
        .await
        .unwrap();

    let link = StoredLink {
        original_url: body.original_url,
        short_code: code.clone(),
        clicks: 0,
    };

    (StatusCode::CREATED, Json(link))
}

//previous implementation of handler redirect using Hashmap
// pub async fn redirect(
//     Path(code): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Redirect, (StatusCode, String)> {
//     let mut map = state.lock().unwrap();
//     match map.get_mut(&code) {
//         Some(link) => {
//             link.clicks += 1;
//             Ok(Redirect::temporary(&link.original_url))
//         }
//         None => Err((StatusCode::NOT_FOUND, String::from("Link not found"))),
//     }
// }
//
//
pub async fn redirect(
    Path(code): Path<String>,
    State(state): State<AppState>,
) -> Result<Redirect, (StatusCode, String)> {
    match sqlx::query_as::<_, StoredLink>("SELECT * FROM links where short_code = ?")
        .bind(&code)
        .fetch_one(&state)
        .await
    {
        Ok(link) => {
            sqlx::query("UPDATE links SET clicks = clicks + 1 WHERE short_code = ?")
                .bind(&code)
                .execute(&state)
                .await
                .unwrap();
            Ok(Redirect::temporary(&link.original_url))
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            String::from("not found link to complete redirection process "),
        )),
    }
}

// previous implementatioon using Hashmap
// pub async fn get_stats(
//     Path(code): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<Json<StoredLink>, (StatusCode, String)> {
//     let map = state.lock().unwrap();
//     match map.get(&code) {
//         Some(link) => Ok(Json(link.clone())),
//         None => Err((StatusCode::NOT_FOUND, String::from("Link not found"))),
//     }
// }
//

pub async fn get_stats(
    Path(code): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<StoredLink>, (StatusCode, String)> {
    match sqlx::query_as::<_, StoredLink>("SELECT * FROM links WHERE short_code = ?")
        .bind(&code)
        .fetch_one(&state)
        .await
    {
        Ok(link) => Ok(Json(link)),
        Err(_) => Err((StatusCode::NOT_FOUND, String::from("link not found"))),
    }
}
// previous HashMap implementation
// pub async fn delete_link(
//     Path(code): Path<String>,
//     State(state): State<AppState>,
// ) -> Result<StatusCode, (StatusCode, String)> {
//     let mut map = state.lock().unwrap();
//     match map.remove(&code) {
//         Some(_) => Ok(StatusCode::NO_CONTENT),
//         None => Err((StatusCode::NOT_FOUND, String::from("link not found"))),
//     }
// }

pub async fn delete_link(
    Path(code): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM links WHERE short_code = ?")
        .bind(&code)
        .execute(&state)
        .await
        .unwrap();
    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            String::from("not found link to delete "),
        ))
    }
}
