use leptos::*;
use leptos::prelude::*;

#[component]
pub fn CurvedDivider(
    #[prop(default = "#000000")] color: &'static str,
    #[prop(default = false)] inverted: bool,
) -> impl IntoView {
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