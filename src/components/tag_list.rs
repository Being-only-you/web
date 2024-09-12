use leptos::*;
use leptos::prelude::*;

#[component]
fn Tag(tag: String) -> impl IntoView {
    view! {
        <a href=format!("/tags/{tag}")>
            <div class="flex flex-1 items-center justify-between truncate rounded-md border border-honest-200 bg-white hover:bg-honest-50 transition duration-150 ease-in-out">
                <div class="flex-1 truncate px-4 py-2">
                    <code class="fluid fluid-0 font-medium text-honest-900 hover:text-honest-500">{tag.clone()}</code>
                </div>
            </div>
        </a>
    }
}

#[component]
pub fn TagList(tags: Vec<String>) -> impl IntoView {
    view! {
        <ul role="list" class="mt-3 grid grid-cols-1 gap-5 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4">
            <For
                each=move || tags.clone()
                key=|tag| tag.clone()
                let:tag
            >
                <li class="col-span-1 rounded-md shadow-sm ">
                    <Tag tag=tag.clone()/>
                </li>
            </For>
        </ul>
    }
}
