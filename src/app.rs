use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
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

        <div class="flex flex-col min-h-screen bg-purple-darker text-white">
            <Router>
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
            </Router>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="container mx-auto px-4 py-16 text-center">
            <h1 class="text-4xl font-bold mb-4 animate-bounce">404 - Page Not Found</h1>
            <p class="text-xl mb-8 animate-fadeIn">Oops! The page you are looking for does not exist.</p>
            <a href="/" class="btn btn-primary animate-pulse">Go back to home</a>
        </div>
    }
}
