use leptos::*;
use leptos::prelude::*;


#[component]
pub fn AnimatedTimeline() -> impl IntoView {
    view! {
        <div class="relative md:flex md:flex-nowrap md:overflow-x-auto md:pb-8">
            <div class="absolute left-4 md:left-1/2 top-0 bottom-0 w-1 md:w-full md:h-1 bg-orange md:transform md:-translate-x-1/2 md:top-1/2"></div>
            {(2023..=2024).map(|year| {
                view! {
                    <div class="relative flex items-center mb-8 md:mb-0 md:flex-col md:w-80 md:flex-shrink-0 md:mx-4 transition-all duration-1000 ease-in-out opacity-0 transform translate-y-4 group">
                        <div class="w-4 h-4 bg-orange rounded-full z-10 absolute left-0 md:left-1/2 md:top-1/2 md:transform md:-translate-x-1/2 md:-translate-y-1/2"></div>
                        <div class="pl-8 md:pl-0 md:pt-8 md:text-center">
                            <h3 class="text-3xl font-bold text-orange mb-2">{year}</h3>
                            <div class="bg-purple-light rounded-lg p-4 shadow-lg transition-all duration-300 group-hover:shadow-xl group-hover:-translate-y-1">
                                <p class="font-bold mb-2">"Pre-registration opens"</p>
                                <p class="text-gray-light">"Pre-registration and username reserving now open. Architecture in place and designs and front-end being developed."</p>
                            </div>
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}