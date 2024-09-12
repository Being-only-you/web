use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [("Home", "/"), ("About", "/about"), ("Posts", "/posts"), ("Tags", "/tags")];

    view! {
        <header class="bg-power-900 text-white shadow-md relative overflow-hidden">
            <div class="absolute top-0 left-0 w-full h-full overflow-hidden z-0">
                <div class="w-64 h-64 bg-power-300 opacity-10 rounded-full absolute -top-32 -left-32 transform rotate-45"></div>
                <div class="w-48 h-48 bg-extroverted-500 opacity-10 skew-x-12 absolute -bottom-24 -right-24"></div>
            </div>
            <nav class="container mx-auto flex items-center justify-between py-4 px-6 relative z-10">
                <a href="/" class="flex items-center space-x-2 transition-all duration-300 ease-in-out transform hover:scale-105">
                    <div class="flex items-center justify-center overflow-hidden">
                        <img
                            class="h-10 w-auto"
                            src="/assets/logo/beu-white-orange.svg"
                            alt="Being You Logo"
                            height="40"
                        />
                    </div>
                    <span class="sr-only">"Being You"</span>
                </a>
                <div class="flex space-x-6">
                    {nav_items
                        .iter()
                        .map(|(name, href)| {
                            view! {
                                <a
                                    class="fluid fluid-0 font-semibold text-white hover:text-extroverted-500 transition-all duration-300 ease-in-out transform hover:scale-105 hover:rotate-3"
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