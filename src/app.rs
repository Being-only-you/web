use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title, Script};
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
        <Title text="Being You - Be Yourself, Uncensored"/>
        <Script type_="importmap">
            "{
                imports: {
                three: 'https://cdn.jsdelivr.net/npm/three/build/three.module.js',
                'three/addons/': 'https://cdn.jsdelivr.net/npm/three/examples/jsm/'
                }
            }"
        </Script>

        <Router>
            <main class="flex flex-col min-h-screen bg-purple-darker text-white">
                <Header/>
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
                <Footer/>
            </main>
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
            <h1 class="text-4xl font-bold mb-4 animate-float">"404 - Page Not Found"</h1>
            <p class="text-xl mb-8 animate-float-delayed">"Oops! The page you're looking for doesn't exist."</p>
            <a href="/" class="btn btn-primary animate-pulse">"Go back to home"</a>
        </div>
    }
}
