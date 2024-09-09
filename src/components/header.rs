use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [("Home", "/"), ("About", "/about"), ("Posts", "/posts"), ("Tags", "/tags")];

    view! {
        <header class="bg-purple-dark text-white shadow-md">
            <nav class="container mx-auto flex items-center justify-between py-4 px-6">
                <a href="/" class="flex items-center space-x-2">
                    <img
                        class="h-10 w-auto rounded-full"
                        src="/images/avatar.webp"
                        alt="Being You Logo"
                        width="40"
                        height="40"
                    />
                    <span class="text-2xl font-bold text-orange">Being You</span>
                </a>
                <div class="flex space-x-6">
                    {nav_items
                        .iter()
                        .map(|(name, href)| {
                            view! {
                                <a
                                    class="text-lg font-semibold text-white hover:text-orange transition duration-300"
                                    href=href.to_string()
                                >
                                    {name.to_string()}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </nav>
        </header>
    }
}