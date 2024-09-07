use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment, ParamSegment
};

use crate::pages::about::*;
use crate::pages::home::*;
use crate::pages::posts::*;
use crate::pages::tags::*;

use crate::components::footer::*;
use crate::components::header::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/beu.css"/>

        // sets the document title
        <Title text="Welcome to BEU"/>

        // content for this welcome page
        <Router>
            <Header/>
            <main>
                <Routes fallback=move || "Page not found.">
                    <Route path=StaticSegment("") view=Home/>
                    <Route path=StaticSegment("about") view=About/>
                    <Route path=StaticSegment("posts") view=Posts/>
                    <Route path=(StaticSegment("posts"), ParamSegment("id")) view=Posts/>
                    <Route path=StaticSegment("tags") view=Tags/>
                    <Route path=(StaticSegment("tags"), ParamSegment("id")) view=Tags/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}

/// 404 - Not Found
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
        <h1>"Not Found"</h1>
    }
}
