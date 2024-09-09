use leptos_meta::Script;
use leptos::prelude::*;

#[component]
pub fn Globe() -> impl IntoView {
    let (loaded, set_loaded) = signal(false);

    Effect::new(move |_| {
        set_loaded(true);
    });

    view! {
        <Show when=move || loaded.get()>
            <div id="globe" class="absolute inset-0 z-0"></div>
            <Script defer="true" type_="module" src="/assets/globe.js"></Script>

        </Show>
    }
}