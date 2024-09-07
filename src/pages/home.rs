use leptos::*;

use crate::components::intro::Intro;
use crate::components::post_list::*;
use crate::content::{get_all_posts, PostStatus, StatusFilter};

#[component]
pub fn Home() -> impl IntoView {
    let published_filter = StatusFilter {
        only: Some(PostStatus::Published),
        exclude: None,
        include: None
    };
    view! {
        <Intro/>
        <PostList posts=get_all_posts(published_filter) />
    }
}