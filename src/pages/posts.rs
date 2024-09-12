use crate::components::post_content::*;
use crate::components::post_list::*;
use crate::content::{get_all_posts, get_post, StatusFilter, PostStatus};
use leptos::*;
use leptos_meta::*;
use leptos_router::hooks::use_params_map;
use leptos::prelude::*;

#[component]
pub fn Posts() -> impl IntoView {
    let published_filter = StatusFilter {
        only: Some(PostStatus::Published),
        exclude: None,
        include: None
    };
    view! {
        <div class="container mx-auto px-4 text-center">
            <h1 class="fluid fluid-4 font-bold tracking-tight text-professional-100">
                "Posts"
            </h1>
        </div>
        <PostList posts=get_all_posts(published_filter)/>
    }
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").unwrap_or_default());
    let post = get_post(id());

    view! {
        <Title text=post.metadata.title.clone()/>

        <div class="container mx-auto px-4 py-4">
            <div class="text-center">
                <p class="fluid fluid-0 font-semibold leading-7 text-white">
                    // TODO DRY
                    {chrono::DateTime::parse_from_rfc3339(&post.metadata.date)
                        .unwrap()
                        .format("%e %b %Y")
                        .to_string()}
                </p>
                <h1 class="mt-2 fluid fluid-4 font-bold tracking-tight text-professional-100">
                    {post.metadata.title.clone()}
                </h1>
            </div>

            <PostContent content=post.content/>
        </div>
    }
}