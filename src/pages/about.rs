use crate::components::post_content::*;
use crate::content::markdown_to_html;
use leptos::*;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 mb-2 fluid fluid-0">
            <PostContent content=markdown_to_html(include_str!("../../content/about.md").to_string())/>
        </div>
    }
}