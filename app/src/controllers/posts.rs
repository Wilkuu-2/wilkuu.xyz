use axum::{
    extract::{Form, Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}
};
use tracing::error;

use crate::{view, AppState, Router};
use entity::post::ActiveModel as PostA;
use entity::post::Column as PostC;
use entity::post::Entity as Post;
use sea_orm::prelude::*;
use sea_orm::ActiveValue;
use sea_orm::QueryOrder;
use axum_named_routes::{NamedRouter, Routes};
use views::posts;

pub fn posts( ) -> Router
{
    NamedRouter::new()
        .route("list","/", get(list))
        .route("edit", "/edit/:id", get(edit))
        .route("add", "/add",get(add))
        .route("update", "/edit",post(update))
        .route("delete", "/delete",post(delete))
}

pub async fn list(
    State(state): State<AppState>,
    routes: Routes,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let p = Post::find().order_by_desc(PostC::Id).all(&state.conn).await;

    match p {
        Ok(posts) => view(&posts::PostList { posts , routes}),
        Err(e) => {
            error!("posts::lists: Error {:?}", e); 
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn edit(
    State(state): State<AppState>,
    routes: Routes, 
    Path(id): Path<u16>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let post = Post::find_by_id(id).one(&state.conn).await;
    match post {
        Ok(p) => match p {
            Some(p) => view(&posts::PostEdit { post_opt: Some(p), routes }),
            None => Err(axum::http::StatusCode::NOT_FOUND),
        },
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add(
    routes: Routes,
) -> Result<impl IntoResponse, axum::http::StatusCode> 
{
    view(&posts::PostEdit { post_opt: None, routes })
}

#[derive(serde::Deserialize)]
pub struct PostEditData {
    id: u16,
    name: String,
    text: String,
}

pub async fn update(
    State(state): State<AppState>,
    Form(data): Form<PostEditData>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let post_get = match data.id {
        0 => Ok(PostA {
            ..Default::default()
        }),
        i => match Post::find_by_id(i).one(&state.conn).await {
            Ok(p) => match p {
                Some(p) => Ok(p.into()),
                None => Err(axum::http::StatusCode::NOT_FOUND),
            },
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    if let Err(e) = post_get {
        return Err(e);
    } // We can safely unwrap post_get

    let mut post_active = post_get.unwrap();
    post_active.title = ActiveValue::set(data.name);
    post_active.text = ActiveValue::set(data.text);
    post_active.modified_at = ActiveValue::set(chrono::Utc::now().naive_local());

    let post = post_active.save(&state.conn).await;

    if let Err(_) = post {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    Ok(axum::response::Redirect::to("/portfolio"))
}


pub async fn delete( 
    State(state): State<AppState>,
    Form(data): Form<PostEditData>
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    match data.id {
        0 => Err(axum::http::StatusCode::BAD_REQUEST),
        i => {
            match Post::delete_by_id(i).exec(&state.conn).await {
                Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
                Ok(dr) => { 
                    match dr.rows_affected {
                        0 => Err(axum::http::StatusCode::BAD_REQUEST),
                        1 => Ok(axum::response::Redirect::to("/portfolio")), 
                        _ => panic!{ "This should not affect more than one row! "},
                    }                    
                } 
            }    
        }
    }
}
