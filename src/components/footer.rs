use chrono::{Datelike, Utc};
use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-power-900 text-white py-12 relative overflow-hidden">
            <div class="absolute top-0 left-0 w-full h-full overflow-hidden z-0">
                <div class="w-80 h-80 bg-power-300 opacity-10 rounded-full absolute -top-40 -left-40 transform rotate-45">
                    <img
                        class="h-10 w-auto"
                        src="/assets/logo/being-you-colour.svg"
                        alt="Being You Logo"
                        height="40"
                    />
                </div>
                <div class="w-96 h-96 bg-extroverted-500 opacity-5 skew-y-12 absolute -bottom-48 -right-48"></div>
            </div>
            <div class="container mx-auto px-6 relative z-10">
                <div class="flex flex-col md:flex-row justify-between items-center">
                    <div class="mb-8 md:mb-0 transition-all duration-300 ease-in-out transform hover:scale-105">
                        <h3 class="fluid fluid-3 font-bold text-extroverted-500 mb-2">
                            <img
                                class="h-10 w-auto"
                                src="/assets/logo/beu-white-orange.svg"
                                alt="Being You Logo"
                                height="40"
                            />
                        </h3>
                        <p class="fluid fluid--1 text-power-300">"Never stop being you! Don't self-censor in fear of losing out."</p>
                    </div>
                    <div class="flex space-x-6">
                        <a href="/about" class="fluid fluid-0 text-white hover:text-extroverted-500 transition-all duration-300 ease-in-out transform hover:scale-105 hover:rotate-3">"About"</a>
                        <a href="/posts" class="fluid fluid-0 text-white hover:text-extroverted-500 transition-all duration-300 ease-in-out transform hover:scale-105 hover:rotate-3">"Posts"</a>
                        <a href="/tags" class="fluid fluid-0 text-white hover:text-extroverted-500 transition-all duration-300 ease-in-out transform hover:scale-105 hover:rotate-3">"Tags"</a>
                    </div>
                </div>
                <hr class="border-power-300 my-8 opacity-30" />
                <div class="text-center">
                    <p class="fluid fluid--1 text-honest-300 transition-all duration-300 ease-in-out hover:text-white">
                        {format!("© {} Being You. All rights reserved.", Utc::now().year())}
                    </p>
                    <p class="fluid fluid-0 text-extroverted-500 mt-2 transition-all duration-300 ease-in-out transform hover:scale-105">
                        "You are not the product, you are the customer."
                    </p>
                </div>
            </div>
        </footer>
    }
}