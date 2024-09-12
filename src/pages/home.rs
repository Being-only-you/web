use leptos::*;
use leptos::prelude::*;
use crate::components::globe::Globe;

#[component]
fn CurvedDivider(color: &'static str, inverted: bool) -> impl IntoView {
    view! {
        <div class={move || if inverted { "transform rotate-180" } else { "" }}>
            <svg viewBox="0 0 1440 100" preserveAspectRatio="none" class="w-full h-[50px]">
                <path 
                    fill={color}
                    d="M0,0 L60,10 C120,20,240,40,360,50 C480,60,600,60,720,50 C840,40,960,20,1080,10 C1200,0,1320,0,1380,0 L1440,0 L1440,100 L0,100 Z"
                ></path>
            </svg>
        </div>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-b from-purple-darker to-purple-dark text-white overflow-hidden">
            // Dynamic Header with Interactive Globe
            <section class="relative h-minusHeader flex items-center justify-center overflow-hidden">
                <Globe/>
                <div class="relative z-10 text-center">
                    <h1 class="text-7xl font-bold mb-4 animate-float">"We are"</h1>
                    <img
                            class="h-20 w-auto"
                            src="/assets/being-you-colour.svg"
                            alt="Being You Logo"
                        />
                    <p class="text-3xl font-light text-orange animate-float">"The social way to work"</p>
                </div>
            </section>

            <CurvedDivider color="#191655" inverted=true />
            <CurvedDivider color="#0D0B2A" inverted=false />
            <CurvedDivider color="#fff" inverted=false />

            // Main Content
            <main>
                // About Section
                <section class="py-32 bg-white text-purple relative overflow-hidden">
                    <div class="container mx-auto px-4 relative z-20">
                        <h2 class="text-5xl font-bold mb-8 text-center animate-float">"About Being You"</h2>
                        <p class="text-xl text-center max-w-3xl mx-auto mb-12 animate-float-delayed">
                            "Being You is a revolutionary platform that combines uncensored social networking with professional growth opportunities. Express yourself freely and advance your career, all in one place."
                        </p>
                        <div class="flex flex-wrap justify-center gap-8">
                            <div class="w-64 h-64 bg-gradient-to-br from-red to-orange p-4 text-center text-white rounded-full flex items-center justify-center text-2xl font-bold animate-float shadow-lg hover:shadow-2xl transition-shadow duration-300">
                                "Zero ads on personal profiles"
                            </div>
                            <div class="w-64 h-64 bg-gradient-to-br from-blue to-purple p-4 text-center text-white rounded-full flex items-center justify-center text-2xl font-bold animate-float-delayed shadow-lg hover:shadow-2xl transition-shadow duration-300">
                                "Promoted job listings on professional profiles"
                            </div>
                            <div class="w-64 h-64 bg-gradient-to-br from-green to-teal p-4 text-center text-white rounded-full flex items-center justify-center text-2xl font-bold animate-float shadow-lg hover:shadow-2xl transition-shadow duration-300">
                                "Content creator revenue sharing"
                            </div>
                        </div>
                    </div>
                </section>

                <CurvedDivider color="#fff" inverted=true />
                <CurvedDivider color="#26217F" inverted=false />

                // Features Section with Overlapping Cards
                <section class="py-10  bg-purple-darker  text-white relative overflow-hidden">
                    <div class="container mx-auto px-4 relative z-10">
                        <h2 class="text-5xl font-bold mb-16 text-center animate-float">"Our Unique Features"</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-12">
                            <div class="bg-gradient-to-br from-purple-light to-purple p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:-rotate-2 animate-float">
                                <h3 class="text-3xl font-bold mb-4 text-orange">"Uncensored Social Networking"</h3>
                                <p class="text-xl">"Express yourself freely without fear of AI-driven censorship. Your voice matters, and we're here to amplify it."</p>
                            </div>
                            <div class="bg-gradient-to-br from-purple-light to-purple p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:rotate-2 animate-float-delayed mt-16">
                                <h3 class="text-3xl font-bold mb-4 text-orange">"Professional Growth"</h3>
                                <p class="text-xl">"Showcase your skills, apply for jobs, and connect with clients. Your career advancement starts here."</p>
                            </div>
                            <div class="bg-gradient-to-br from-purple-light to-purple p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:-rotate-2 animate-float">
                                <h3 class="text-3xl font-bold mb-4 text-orange">"Content Creator Support"</h3>
                                <p class="text-xl">"Benefit from our revenue sharing program. Your creativity deserves recognition and rewards."</p>
                            </div>
                            <div class="bg-gradient-to-br from-purple-light to-purple p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:rotate-2 animate-float-delayed mt-16">
                                <h3 class="text-3xl font-bold mb-4 text-orange">"Secure Transactions"</h3>
                                <p class="text-xl">"Our escrow service ensures safe and transparent transactions for all professional engagements."</p>
                            </div>
                        </div>
                    </div>
                </section>

                <CurvedDivider color="#26217F" inverted=true />

                // Development Log Section
                <section class="py-32 bg-white relative overflow-hidden">
                    <div class="container mx-auto px-4 relative z-10">
                        <h2 class="text-5xl font-bold mb-8 text-center animate-float">"Development Log"</h2>
                        <p class="text-center mb-8 animate-float-delayed">
                            "This is where we will write updates on the development of the app. We will aim to update this weekly, however we can't guarantee this will always be the case. If you wish to request an update on the development, please email us."
                        </p>
                    </div>
                    <div class="text-center">
                        <a href="/tags/dev" class="bg-gradient-to-r from-red to-orange text-white text-xl font-bold py-4 px-8 rounded-full transition-all duration-300 ease-in-out transform hover:scale-105 hover:shadow-2xl animate-pulse">
                            "Learn More"
                        </a>
                    </div>
                </section>
            </main>
        </div>
    }
}