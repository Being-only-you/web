use leptos::*;
use leptos::prelude::*;

use crate::components::post_list::*;
use crate::content::{get_all_posts, PostStatus, StatusFilter};

#[component]
pub fn Home() -> impl IntoView {
    let published_filter = StatusFilter {
        only: Some(PostStatus::Published),
        exclude: None,
        include: None
    };

    view! {
        <div class="min-h-screen bg-purple-darker text-white">
            <section class="py-20 animate-fadeIn">
                <div class="container text-center">
                    <h1 class="text-6xl font-bold mb-4 animate-bounce">Being You</h1>
                    <p class="text-3xl font-light text-purple-light animate-pulse">The new app to be you</p>
                    <p class="text-xl mt-6 max-w-2xl mx-auto text-gray-light animate-fadeIn">
                        "Never stop being you! Don't self-censor in fear of losing out."
                    </p>
                </div>
            </section>

            <section class="py-20 bg-purple animate-slideUp">
                <div class="container">
                    <h2 class="text-4xl font-bold mb-8 text-center animate-bounce">About Being You</h2>
                    <div class="flex flex-col md:flex-row items-center justify-between">
                        <div class="md:w-1/2 mb-8 md:mb-0 animate-fadeIn">
                            <p class="text-xl mb-6 text-purple-light">
                                Being You is a revolutionary social media platform that combines uncensored expression with professional networking. We believe in your right to be yourself, both in your personal life and your career.
                            </p>
                            <p class="text-xl text-purple-light">
                                Our platform is designed to empower you in both your personal and professional life.
                            </p>
                        </div>
                        <div class="md:w-1/2 flex justify-center">
                            <div class="w-64 h-64 bg-red rounded-full flex items-center justify-center text-2xl font-bold animate-ping">
                                "You are the customer, not the product"
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-20 animate-fadeIn">
                <div class="container">
                    <h2 class="text-4xl font-bold mb-8 text-center animate-pulse">Our Unique Features</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        <div class="bg-purple p-6 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 animate-fadeIn">
                            <h3 class="text-2xl font-bold mb-4 text-orange">Uncensored Social Networking</h3> 
                            <p class="text-purple-light">Express yourself freely without fear of AI-driven censorship. Your voice matters, and we&apos;re here to amplify it.</p>
                        </div>
                        <div class="bg-purple p-6 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 animate-fadeIn delay-100">
                            <h3 class="text-2xl font-bold mb-4 text-orange">Professional Branding</h3>
                            <p class="text-purple-light">Showcase your skills, portfolio, and achievements. Stand out in the competitive job market with our powerful branding tools.</p>
                        </div>
                        <div class="bg-purple p-6 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 animate-fadeIn delay-200">
                            <h3 class="text-2xl font-bold mb-4 text-orange">Job Application Platform</h3>
                            <p class="text-purple-light">Discover exciting career opportunities and apply directly through our integrated job board. Your next big break is just a click away.</p>
                        </div>
                        <div class="bg-purple p-6 rounded-lg shadow-lg transform hover:scale-105 transition duration-300 animate-fadeIn delay-300">
                            <h3 class="text-2xl font-bold mb-4 text-orange">Freelance Marketplace</h3>
                            <p class="text-purple-light">Connect with clients, showcase your services, and grow your freelance business. Being You is your partner in professional success.</p>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-20 bg-purple animate-slideUp">
                <div class="container text-center">
                    <h2 class="text-4xl font-bold mb-8 animate-bounce">Join Being You Today</h2>
                    <p class="text-xl mb-8 text-purple-light animate-pulse">Start being your true self while advancing your career!</p>
                    <button class="btn btn-primary text-xl animate-pulse hover:animate-bounce">
                        "Get Started Now"
                    </button>
                </div>
            </section>

            <section class="py-20 animate-fadeIn">
                <div class="container">
                    <h2 class="text-4xl font-bold mb-8 text-center animate-pulse">Latest Updates</h2>
                    <PostList posts=get_all_posts(published_filter) />
                </div>
            </section>
        </div>
    }
}