use crate::content::PostData;
use itertools::Itertools;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
fn PostCover(href: String, src: String) -> impl IntoView {
    view! {
        <a href=href>
            <img
                class="absolute inset-0 h-full w-full rounded-2xl bg-honest-50 object-cover"
                src=src
                // TODO
                alt=""
                width="720"
                height="1280"
            />
            <div class="absolute inset-0 rounded-2xl ring-1 ring-inset ring-honest-900/10"></div>
        </a>
    }
}

#[component]
fn TagLabel(tag: String) -> impl IntoView {
    let href = format!("/tags/{}", tag.clone());
    view! {
        <a
            class="inline-flex items-center gap-x-1.5 rounded-full px-2 py-1 fluid fluid--1 font-medium text-honest-900 ring-1 ring-inset ring-honest-200"
            href=href
        >
            {tag}
        </a>
    }
}

#[component]
fn PostPreview(id: String, post: PostData) -> impl IntoView {
    view! {
        <article class="relative isolate flex flex-col gap-8 lg:flex-row">
            {post.metadata.cover.as_ref().map(|cover| view! {
                <div class="relative aspect-[16/9] lg:w-64 lg:shrink-0">
                    <PostCover href=format!("/posts/{}", id.clone()) src=cover.clone()/>
                </div>
            })}
            <div>
                <div class="flex items-center gap-x-4 fluid fluid--1">
                    <p>
                        {chrono::DateTime::parse_from_rfc3339(&post.metadata.date)
                            .unwrap()
                            .format("%e %b %Y")
                            .to_string()}
                    </p>
                    <For
                        each=move || post.metadata.tags.clone().into_iter()
                        key=|tag| tag.clone()
                        let:tag
                    >
                        <TagLabel tag=tag.clone()/>
                    </For>
                </div>
                <div class="group relative max-w-xl">
                    <h2 class="mt-3 fluid fluid-2 font-semibold leading-6 text-honest-100 group-hover:text-honest-500">
                        <a href=format!("/posts/{}", id.clone())>
                            <span class="absolute inset-0"></span>
                            {post.metadata.title}
                        </a>
                    </h2>
                    <div
                        class="prose mt-5 fluid fluid-0 leading-6 text-honest-500"
                        inner_html=post.metadata.summary
                    ></div>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn PostList(posts: HashMap<String, PostData>) -> impl IntoView {
    view! {
        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <div class="mt-16 space-y-10 lg:mt-10 lg:space-y-10">
                <For
                    each=move || posts.clone().into_iter().sorted_by(|(_, a), (_, b)| b.metadata.date.cmp(&a.metadata.date))
                    key=|post| post.0.clone()
                    let:post
                >
                    <li class="col-span-1 rounded-md shadow-sm list-none">
                        <PostPreview id=post.0.clone() post=post.1.clone()/>
                    </li>
                </For>
            </div>
        </div>
    }
}