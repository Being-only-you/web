use leptos::*;
use leptos::prelude::*;

#[component]
pub fn PostContent(content: String) -> impl IntoView {
    view! { <div class="prose max-w-none" inner_html=content></div> }
}