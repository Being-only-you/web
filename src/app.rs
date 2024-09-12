use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title, Link, Meta, Script};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, ParamSegment
};

use crate::pages::about::*;
use crate::pages::home::*;
use crate::pages::posts::*;
use crate::pages::tags::*;

use crate::components::footer::*;
use crate::components::header::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/beu.css"/>
        <Title text="Being You - Be Yourself, Uncensored | Social Networking Platform"/>
        <Meta name="description" content="Being You is a revolutionary platform combining uncensored social networking with professional growth opportunities. Express yourself freely and advance your career."/>
        <Meta name="keywords" content="social network, professional networking, free speech, privacy, career development"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta property="og:title" content="Being You - Be Yourself, Uncensored"/>
        <Meta property="og:description" content="Being You is a revolutionary platform combining uncensored social networking with professional growth opportunities. Express yourself freely and advance your career."/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:url" content="https://beingyou.uk"/>
        <Meta property="og:image" content="/assets/logo/being-you-colour.svg"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content="Being You - Be Yourself, Uncensored"/>
        <Meta name="twitter:description" content="Being You is a revolutionary platform combining uncensored social networking with professional growth opportunities. Express yourself freely and advance your career."/>
        <Meta name="twitter:image" content="/assets/logo/being-you-colour.svg"/>
        <Link rel="canonical" href="https://beingyou.uk"/>
        <Script type_="importmap">
            "{
                imports: {
                three: 'https://cdn.jsdelivr.net/npm/three/build/three.module.js',
                'three/addons/': 'https://cdn.jsdelivr.net/npm/three/examples/jsm/'
                }
            }"
        </Script>

        <Router>
            <div class="flex flex-col min-h-screen bg-power-900 text-white">
                <Header/>
                <main class="flex-grow">
                    <Routes fallback=move || {
                        view! { <NotFound /> }
                    }>
                        <Route path=StaticSegment("") view=Home/>
                        <Route path=StaticSegment("about") view=About/>
                        <Route path=StaticSegment("posts") view=Posts/>
                        <Route path=(StaticSegment("posts"), ParamSegment("id")) view=Post/>
                        <Route path=StaticSegment("tags") view=Tags/>
                        <Route path=(StaticSegment("tags"), ParamSegment("id")) view=Tag/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="container mx-auto px-4 py-16 text-center">
            <h1 class="fluid fluid-4 font-bold mb-4 animate-float">"404 - Page Not Found"</h1>
            <p class="fluid fluid-1 mb-8 animate-float-delayed">"Oops! The page you're looking for doesn't exist."</p>
            <a href="/" class="btn btn-primary animate-pulse fluid fluid-0">"Go back to home"</a>
        </div>
    }
}
