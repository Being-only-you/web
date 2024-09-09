use chrono::{Datelike, Utc};
use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-purple-dark text-white py-12 relative overflow-hidden">
            <div class="absolute top-0 left-0 w-full h-full overflow-hidden z-0">
                <div class="w-80 h-80 bg-purple-light opacity-10 rounded-full absolute -top-40 -left-40 transform rotate-45"></div>
                <div class="w-96 h-96 bg-orange opacity-5 skew-y-12 absolute -bottom-48 -right-48"></div>
            </div>
            <div class="container mx-auto px-6 relative z-10">
                <div class="flex flex-col md:flex-row justify-between items-center">
                    <div class="mb-8 md:mb-0 transition-all duration-300 ease-in-out transform hover:scale-105">
                        <h3 class="text-3xl font-bold text-orange mb-2">Being You</h3>
                        <p class="text-sm text-purple-light">&ldquo;Never stop being you! Don&rsquo;t self-censor in fear of losing out.&rdquo;</p>
                    </div>
                    <div class="flex space-x-6">
                        <a href="/about" class="text-white hover:text-orange transition-all duration-300 ease-in-out transform hover:scale-105 hover:rotate-3">About</a>
                        <a href="/posts" class="text-white hover:text-orange transition-all duration-300 ease-in-out transform hover:scale-105 hover:rotate-3">Posts</a>
                        <a href="/tags" class="text-white hover:text-orange transition-all duration-300 ease-in-out transform hover:scale-105 hover:rotate-3">Tags</a>
                    </div>
                </div>
                <hr class="border-purple-light my-8 opacity-30" />
                <div class="text-center">
                    <p class="text-sm text-gray-light transition-all duration-300 ease-in-out hover:text-white">
                        &copy; {Utc::now().year()} Being You. All rights reserved.
                    </p>
                    <p class="text-sm text-orange mt-2 transition-all duration-300 ease-in-out transform hover:scale-105">
                        &ldquo;You are not the product, you are the customer.&rdquo;
                    </p>
                </div>
            </div>
        </footer>
    }
}