use leptos::*;
use leptos::prelude::*;

use crate::components::post_list::*;
use crate::components::globe::Globe;
use crate::content::{get_all_posts, PostStatus, StatusFilter};

#[component]
pub fn Home() -> impl IntoView {
    let published_filter = StatusFilter {
        only: Some(PostStatus::Published),
        exclude: None,
        include: None
    };

    view! {
        <div class="min-h-screen bg-gradient-to-b from-purple-darker to-purple-dark text-white overflow-hidden">
            // Dynamic Header with Interactive Globe
            <header class="relative h-screen flex items-center justify-center overflow-hidden">
                <Globe/>
                <div class="relative z-10 text-center">
                    <h1 class="text-7xl font-bold mb-4 animate-float">Being You</h1>
                    <p class="text-3xl font-light animate-float-delayed">The new way to be you</p>
                </div>
            </header>

            // Main Content
            <main>
                // About Section with Curved Overlay
                <section class="py-32 relative overflow-hidden">
                    <div class="container mx-auto px-4 relative z-20">
                        <h2 class="text-5xl font-bold mb-8 text-center animate-float">About Being You</h2>
                        <p class="text-xl text-center max-w-3xl mx-auto mb-12 animate-float-delayed">
                            Being You is a revolutionary platform that combines uncensored social networking with professional growth opportunities. Express yourself freely and advance your career, all in one place.
                        </p>
                        <div class="flex flex-wrap justify-center gap-8">
                            <div class="w-64 h-64 bg-gradient-to-br from-red to-orange rounded-full flex items-center justify-center text-2xl font-bold animate-float shadow-lg hover:shadow-2xl transition-shadow duration-300">
                                "Zero ads on personal profiles"
                            </div>
                            <div class="w-64 h-64 bg-gradient-to-br from-blue to-purple rounded-full flex items-center justify-center text-2xl font-bold animate-float-delayed shadow-lg hover:shadow-2xl transition-shadow duration-300">
                                "Promoted job listings on professional profiles"
                            </div>
                            <div class="w-64 h-64 bg-gradient-to-br from-green to-teal rounded-full flex items-center justify-center text-2xl font-bold animate-float shadow-lg hover:shadow-2xl transition-shadow duration-300">
                                "Content creator revenue sharing"
                            </div>
                        </div>
                    </div>
                    <div class="absolute bottom-0 left-0 w-full h-1/2 bg-purple z-10 transform -skew-y-6"></div>
                </section>

                // Features Section with Overlapping Cards
                <section class="py-32 bg-purple relative overflow-hidden">
                    <div class="container mx-auto px-4 relative z-10">
                        <h2 class="text-5xl font-bold mb-16 text-center animate-float">Our Unique Features</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-12 -mt-16">
                            <div class="bg-gradient-to-br from-purple-light to-purple p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:-rotate-2 animate-float">
                                <h3 class="text-3xl font-bold mb-4 text-orange">Uncensored Social Networking</h3>
                                <p class="text-xl">Express yourself freely without fear of AI-driven censorship. Your voice matters, and we&rsquo;re here to amplify it.</p>
                            </div>
                            <div class="bg-gradient-to-br from-purple-light to-purple p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:rotate-2 animate-float-delayed mt-16">
                                <h3 class="text-3xl font-bold mb-4 text-orange">Professional Growth</h3>
                                <p class="text-xl">Showcase your skills, apply for jobs, and connect with clients. Your career advancement starts here.</p>
                            </div>
                            <div class="bg-gradient-to-br from-purple-light to-purple p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:-rotate-2 animate-float">
                                <h3 class="text-3xl font-bold mb-4 text-orange">Content Creator Support</h3>
                                <p class="text-xl">Benefit from our revenue sharing program. Your creativity deserves recognition and rewards.</p>
                            </div>
                            <div class="bg-gradient-to-br from-purple-light to-purple p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:rotate-2 animate-float-delayed mt-16">
                                <h3 class="text-3xl font-bold mb-4 text-orange">Secure Transactions</h3>
                                <p class="text-xl">Our escrow service ensures safe and transparent transactions for all professional engagements.</p>
                            </div>
                        </div>
                    </div>
                    <div class="absolute top-0 left-0 w-full h-1/2 bg-purple-darker z-0 transform skew-y-6"></div>
                    <div class="absolute bottom-0 left-0 w-full h-1/2 bg-purple-darker z-0 transform -skew-y-6"></div>
                </section>

                // Call to Action Section with Curved Background
                <section class="py-32 relative overflow-hidden bg-gradient-to-b from-purple-darker to-purple">
                    <div class="container mx-auto px-4 text-center relative z-10">
                        <h2 class="text-5xl font-bold mb-8 animate-float">Join Being You Today</h2>
                        <p class="text-2xl mb-12 animate-float-delayed">Start being your true self while advancing your career!</p>
                        <button class="bg-gradient-to-r from-red to-orange text-white text-xl font-bold py-4 px-8 rounded-full transition-all duration-300 ease-in-out transform hover:scale-105 hover:shadow-2xl animate-pulse">
                            Get Started Now
                        </button>
                    </div>
                    <div class="absolute top-0 left-0 w-full">
                        <svg viewBox="0 0 1440 320" xmlns="http://www.w3.org/2000/svg">
                            <path fill="#26217F" fill-opacity="1" d="M0,64L48,80C96,96,192,128,288,128C384,128,480,96,576,90.7C672,85,768,107,864,128C960,149,1056,171,1152,176C1248,181,1344,171,1392,165.3L1440,160L1440,0L1392,0C1344,0,1248,0,1152,0C1056,0,960,0,864,0C768,0,672,0,576,0C480,0,384,0,288,0C192,0,96,0,48,0L0,0Z"></path>
                        </svg>
                    </div>
                    <div class="absolute bottom-0 left-0 w-full transform rotate-180">
                        <svg viewBox="0 0 1440 320" xmlns="http://www.w3.org/2000/svg">
                            <path fill="#26217F" fill-opacity="1" d="M0,64L48,80C96,96,192,128,288,128C384,128,480,96,576,90.7C672,85,768,107,864,128C960,149,1056,171,1152,176C1248,181,1344,171,1392,165.3L1440,160L1440,0L1392,0C1344,0,1248,0,1152,0C1056,0,960,0,864,0C768,0,672,0,576,0C480,0,384,0,288,0C192,0,96,0,48,0L0,0Z"></path>
                        </svg>
                    </div>
                </section>

                // Latest Updates Section with Overlapping Background
                <section class="py-32 bg-purple-darker relative overflow-hidden">
                    <div class="container mx-auto px-4 relative z-10">
                        <h2 class="text-5xl font-bold mb-16 text-center animate-float">Latest Updates</h2>
                        <PostList posts=get_all_posts(published_filter) />
                    </div>
                    <div class="absolute top-0 left-0 w-full h-1/2 bg-purple z-0 transform -skew-y-6"></div>
                </section>
            </main>
        </div>
    }
}