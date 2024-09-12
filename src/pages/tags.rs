use crate::components::post_list::*;
use crate::components::tag_list::*;
use crate::content::{get_all_tags, get_posts_by_tag};
use leptos::*;
use leptos_router::hooks::use_params_map;
use leptos::prelude::*;
use urlencoding::decode;

#[component]
pub fn Tags() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4">
            <h1 class="mt-2 py-20 text-center fluid fluid-4 font-bold tracking-tight rounded-lg rounded-tr-none bg-honest-100 text-power-500">
                "Tags"
            </h1>
            <TagList tags=get_all_tags()/>
        </div>
    }
}

#[component]
pub fn Tag() -> impl IntoView {
    let params = use_params_map();
    let tag = move || {
        params.read()
            .get("id")
            .map(|t| decode(&t).expect("UTF-8").into_owned())
            .unwrap_or_default()
    };
    let post_list = move || {
        view! { <PostList posts=get_posts_by_tag(tag())/>}
    };

    view! {
        <h1 class="container mx-auto px-4 text-center fluid fluid-4 font-bold tracking-tight text-honest-100">
            <code class="fluid fluid-3">{tag}</code>
        </h1>
        {post_list}
    }
}