use askama::Template;
use axum_named_routes::Routes; 
use entity::post::Model as Post;
use crate::filters;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostList {
    pub posts: Vec<Post>,
    pub routes: Routes, 
}

#[derive(Template)]
#[template(path = "post_edit.html")]
pub struct PostEdit {
    pub post_opt: Option<Post>,
    pub routes: Routes, 
}
