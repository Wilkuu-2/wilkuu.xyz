use axum::{
    extract::{State, Path, Form},
    response::IntoResponse,
    routing::{get, post}}; 

use crate::{view, AppState, Router}; 
use views::posts; 
use sea_orm::prelude::*; 
use sea_orm::ActiveValue;
use sea_orm::QueryOrder;
use entity::post::Entity as Post; 
use entity::post::ActiveModel as PostA; 
use entity::post::Column as PostC;

pub fn posts() -> Router{
    Router::new()
        .route("/", get(list))
        .route("/edit/:id", get(edit))
        .route("/add",  get(add))
        .route("/edit", post(update))
} 

pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let posts = Post::find().order_by_desc(PostC::Id).all(&state.conn).await; 

    match posts {
        Ok(p) => view(&posts::PostList{posts: p}),
        Err(_) => Err(axum::http::StatusCode::NO_CONTENT), 
    }
}

pub async fn edit(State(state): State<AppState>, Path(id): Path<u16> ) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let post = Post::find_by_id(id).one(&state.conn).await; 
    match post {
        Ok(p) => match p { 
            Some(p) => view(&posts::PostEdit{post_opt: Some(p)}), 
            None    => Err(axum::http::StatusCode::NOT_FOUND)
        },
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    } 
}

pub async fn add() ->Result<impl IntoResponse, axum::http::StatusCode> 
{
    view(&posts::PostEdit{post_opt: None}) 
}

#[derive(serde::Deserialize)]
pub struct PostEditData {
    id: u16, 
    name: String, 
    text: String
} 
    
pub async fn update(State(state): State<AppState>, Form(data): Form<PostEditData>) -> Result<impl IntoResponse, axum::http::StatusCode>
{
    let post_get = match data.id {
        0 => {Ok(PostA { ..Default::default() } )}
        i => match Post::find_by_id(i).one(&state.conn).await {
            Ok(p) =>{match p {
                Some(p) => Ok(p.into()),
                None => Err(axum::http::StatusCode::NOT_FOUND)
            }}, 
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR) 
        } 
    }; 
    
    if let Err(e) = post_get {
        return Err(e)
    } // We can safely unwrap post_get 

    let mut post_active = post_get.unwrap();
    post_active.title = ActiveValue::set(data.name);
    post_active.text = ActiveValue::set(data.text);

    let post = post_active.save(&state.conn).await; 
    
    if let Err(_) = post{
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    Ok(axum::response::Redirect::to("/portfolio"))
}
