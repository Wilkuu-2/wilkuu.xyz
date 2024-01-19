use askama::Template;

use entity::post::Model as Post;

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostList {
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post_edit.html")]
pub struct PostEdit {
    pub post_opt: Option<Post>,
}
