use leptos::*;
use leptos::prelude::*;
use crate::components::curved_divider::CurvedDivider;

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="min-h-minusHeader container mx-auto flex flex-col justify-center items-center py-12 px-6 overflow-hidden relative">
            <div class="absolute inset-0 bg-[url('/assets/images/every-you.png')] bg-cover bg-center opacity-10 z-0"></div>
            <div class="relative z-10 text-center flex flex-col items-center max-w-4xl">
                <h1 class="fluid fluid-5 font-bold mb-4 animate-float">"We are"</h1>
                <img
                    class="w-auto h-full max-h-xs mb-6"
                    src="/assets/logo/being-you-orange-white.svg"
                    alt="Being You Logo"
                />
                <p class="fluid fluid-3 font-light text-extroverted-500 animate-float mb-6">"The social way to work"</p>
                <p class="fluid fluid-1 mb-8">
                    "Free speech + privacy focus platform, designed to connect to each other on a social and professional level. We
                    developed this for you to work as a tool for the development of your future professionally, while providing you with a
                    ad free social networking application for the social you!"
                </p>
                <div>
                    <a href="/about" class="bg-extroverted-500 text-white rounded-full rounded-tr-none px-8 py-2 fluid fluid-0">
                        "Learn more"
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn AboutSection() -> impl IntoView {
    view! {
        <section class="py-32 bg-white text-power-900 relative overflow-hidden">
            <div class="container mx-auto px-4 relative z-20">
                <h2 class="fluid fluid-4 font-bold mb-8 text-center animate-float">"About Being You"</h2>
                <p class="fluid fluid-1 text-center max-w-3xl mx-auto mb-12 animate-float-delayed">
                    "Being You is a revolutionary platform that combines uncensored social networking with professional growth opportunities. Express yourself freely and advance your career, all in one place."
                </p>
                <div class="flex flex-wrap justify-center gap-8">
                    <div class="w-64 h-64 bg-gradient-to-br from-extroverted-500 to-extroverted-500 p-4 text-center text-white rounded-full flex items-center justify-center fluid fluid-0 font-bold animate-float shadow-lg hover:shadow-2xl transition-shadow duration-300">
                        "Zero ads on personal profiles"
                    </div>
                    <div class="w-64 h-64 bg-gradient-to-br from-social-500 to-power-500 p-4 text-center text-white rounded-full flex items-center justify-center fluid fluid-0 font-bold animate-float-delayed shadow-lg hover:shadow-2xl transition-shadow duration-300">
                        "Promoted job listings on professional profiles"
                    </div>
                    <div class="w-64 h-64 bg-gradient-to-br from-professional-500 to-professional-300 p-4 text-center text-white rounded-full flex items-center justify-center fluid fluid-0 font-bold animate-float shadow-lg hover:shadow-2xl transition-shadow duration-300">
                        "Content creator revenue sharing"
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeaturesSection() -> impl IntoView {
    view! {
        <section class="py-10 bg-power-900 text-white relative overflow-hidden">
            <div class="container mx-auto px-4 relative z-10">
                <h2 class="fluid fluid-4 font-bold mb-16 text-center animate-float">"Our Unique Features"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-12">
                    <div class="bg-gradient-to-br from-power-300 to-power-500 p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:-rotate-2 animate-float">
                        <h3 class="fluid fluid-3 font-bold mb-4 text-extroverted-500">"Uncensored Expression"</h3>
                        <p class="fluid fluid-1">"Express yourself freely without fear of AI-driven censorship. Your voice matters, and we're here to amplify it."</p>
                    </div>
                    <div class="bg-gradient-to-br from-power-300 to-power-500 p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:rotate-2 animate-float-delayed mt-16">
                        <h3 class="fluid fluid-3 font-bold mb-4 text-extroverted-500">"Professional Branding"</h3>
                        <p class="fluid fluid-1">"Showcase your skills, apply for jobs, and connect with clients. Your career advancement starts here."</p>
                    </div>
                    <div class="bg-gradient-to-br from-power-300 to-power-500 p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:-rotate-2 animate-float">
                        <h3 class="fluid fluid-3 font-bold mb-4 text-extroverted-500">"Connecting People"</h3>
                        <p class="fluid fluid-1">"Build meaningful relationships with friends, family, and professional contacts in a censorship-free environment."</p>
                    </div>
                    <div class="bg-gradient-to-br from-power-300 to-power-500 p-8 rounded-3xl shadow-xl transition-all duration-500 ease-in-out transform hover:scale-105 hover:rotate-2 animate-float-delayed mt-16">
                        <h3 class="fluid fluid-3 font-bold mb-4 text-extroverted-500">"Business Tools"</h3>
                        <p class="fluid fluid-1">"Access job boards, freelancing opportunities, and business management tools to grow your professional network."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn BenefitsSection() -> impl IntoView {
    view! {
        <section class="py-20 w-full bg-white text-power-900 relative">
            
            <div class="container px-4 mx-auto relative z-10">
                <h2 class="fluid fluid-4 font-bold mb-12 text-center">Our Benefits</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <div class="flex flex-col items-center text-center p-6 bg-honest-200 rounded-lg rounded-tr-none border border-power-100 shadow-md transition-all duration-300 ease-in-out hover:animate-expand focus:animate-expand active:animate-expand">
                        <span class="flex w-16 h-16 items-center justify-center mb-4">
                            <img class="w-full" src="/assets/iconography/light-mode/expression.svg" alt="Expression" />
                        </span>
                        <h3 class="fluid fluid-3 font-bold mb-4">"Freedom to be yourself!"</h3>
                        <p class="fluid fluid-0 text-honest-500">
                            "We live to help empower you to be you and not the woke you! We will ensure you can freely express yourself in a Law-abiding way without the risk of censorship! It's only when we challenge a native and express our views. Can a civil society grow as one!"
                        </p>
                    </div>
                    <div class="flex flex-col items-center text-center p-6 bg-honest-200 border border-power-100 rounded-lg rounded-tr-none shadow-md transition-all duration-300 ease-in-out hover:animate-expand focus:animate-expand active:animate-expand">
                        <span class="flex w-16 h-16 items-center justify-center mb-4">
                            <img class="w-full" src="/assets/iconography/light-mode/calendar.svg" alt="calendar" />
                        </span>
                        <h3 class="fluid fluid-3 font-bold mb-4">"Professional you? We have you covered too!"</h3>
                        <p class="fluid fluid-0 text-honest-500">
                            "Showcase your work, skills, and talents to potential employers and clients. Our platform provides tools for personal branding and professional networking, helping you advance your career."
                        </p>
                    </div>
                    <div class="flex flex-col items-center text-center p-6 bg-honest-200 border border-power-100 rounded-lg rounded-tr-none  shadow-md transition-all duration-300 ease-in-out hover:animate-expand focus:animate-expand active:animate-expand">
                        <span class="flex w-16 h-16 items-center justify-center mb-4">
                            <img class="w-full" src="/assets/iconography/light-mode/privacy.svg" alt="Privacy" />
                        </span>
                        <h3 class="fluid fluid-3 font-bold mb-4">"Your privacy is our concern!"</h3>
                        <p class="fluid fluid-0 text-honest-500">
                            "You are a "<span class="italic uppercase">"user"</span> " and not a "<span class="italic uppercase">"customer."</span>" We promise we will never sell or provide 3rd party access to your private information! Your privacy and right to express yourself freely are our top priorities."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn DevelopmentLogSection() -> impl IntoView {
    view! {
        <section class="py-32 relative overflow-hidden">
            <div class="absolute inset-0 bg-[url('/assets/images/connecting.svg')] bg-cover bg-center opacity-5 z-0"></div>
            <div class="container mx-auto px-4 relative z-10">
                <h2 class="fluid fluid-4 font-bold mb-8 text-center animate-float">"Development Log"</h2>
                <p class="fluid fluid-1 text-center mb-8 animate-float-delayed">
                    "This is where we will write updates on the development of the app. We will aim to update this weekly, however we can't guarantee this will always be the case. If you wish to request an update on the development, please email us."
                </p>
            </div>
            <div class="text-center">
                <a href="/tags/dev" class="bg-gradient-to-r from-extroverted-500 to-extroverted-500 text-white fluid fluid-0 font-bold py-4 px-8 rounded-full rounded-tr-none transition-all duration-300 ease-in-out transform hover:scale-105 hover:shadow-2xl animate-pulse">
                    "Learn More"
                </a>
            </div>
        </section>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-b from-power-700 to-power-900 text-white overflow-hidden">
            <Hero />

            <CurvedDivider color="#26217F" inverted=true />
            <CurvedDivider color="#191655" inverted=false />
            <CurvedDivider color="#fff" inverted=false />

            <main>
                <AboutSection />

                <CurvedDivider color="#fff" inverted=true />
                <CurvedDivider color="#0D0B2A" inverted=false />

                <FeaturesSection />

                <CurvedDivider color="#0D0B2A" inverted=true />
                <CurvedDivider color="#fff" inverted=false />

                <BenefitsSection />

                <CurvedDivider color="#fff" inverted=true />
                <CurvedDivider color="#191655" inverted=false />

                <DevelopmentLogSection />
            </main>
        </div>
    }
}