use chrono::{Datelike, Utc};
use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-purple-dark text-white py-8">
            <div class="container mx-auto px-6">
                <div class="flex flex-col md:flex-row justify-between items-center">
                    <div class="mb-4 md:mb-0">
                        <h3 class="text-2xl font-bold text-orange mb-2">Being You</h3>
                        <p class="text-sm">"Never stop being you! Don't self-censor in fear of losing out."</p>
                    </div>
                    <div class="flex space-x-4">
                        <a href="/about" class="text-white hover:text-orange transition duration-300">About</a>
                        <a href="/posts" class="text-white hover:text-orange transition duration-300">Posts</a>
                        <a href="/tags" class="text-white hover:text-orange transition duration-300">Tags</a>
                    </div>
                </div>
                <hr class="border-purple-light my-6" />
                <div class="text-center">
                    <p class="text-sm text-gray-light">
                        {format!("© {} Being You. All rights reserved.", Utc::now().year())}
                    </p>
                    <p class="text-sm text-gray-light mt-2">
                        "You are not the product, you are the customer."
                    </p>
                </div>
            </div>
        </footer>
    }
}