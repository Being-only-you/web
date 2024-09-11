use crate::components::post_content::*;
use crate::content::markdown_to_html;
use leptos::*;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-3xl px-6 lg:px-8 mb-2">
            <PostContent content=markdown_to_html(include_str!("../../content/about.md").to_string())/>
        </div>
    }
}