use crate::app_server::{self, get_example_data_from_db, get_error_from_server};
use leptos::{html::button, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};
use crate::app::*;

#[component]
pub fn Sandbox() -> impl IntoView {
    // This sandbox shows basic toggle for local resources (signal)
    let (sig_getter, sig_setter) = signal("default_signal".to_string());
    
    let flowbite_button = "text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none";
    view! {
        <MainLayout>
            <button class=flowbite_button
                on:click=move |ev| {
                    let ets = event_target_value(&ev);
                    leptos::logging::log!("Clicked sandbox 1, ev: {ev:?}, etv: {ets}");
                    sig_setter.set("from sandbox 1".to_string());
                } >
                Sandbox 1
            </button>

            <button class=flowbite_button
                on:click=move |ev| {
                    leptos::logging::log!("Clicked sandbox 2, ev: {ev:?} "); 
                    sig_setter.set("from sandbox 2".to_string());
                } >
                Sandbox 2
            </button>
            <p>{ move || sig_getter.get() }</p>
        </MainLayout>
    }
}

#[component]
pub fn Sandbox2() -> impl IntoView {
    // Showw how to use server resource
    let (resource_counter, resource_counter_set) = signal(0);
    let res = Resource::new(move || {resource_counter.get()},
                            |_| async move {get_example_data_from_db().await}
                           );

    let flowbite_button = "text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none";
    view! {
        <MainLayout>
            <button class=flowbite_button
                on:click=move |_| {
                    *resource_counter_set.write() +=1;
                    let counter_value = resource_counter.get();
                    leptos::logging::log!("Clicked sandbox 2, counter: {counter_value}");
                    
                } >
                Sandbox 2
            </button>

            <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || Suspend::new(async move {
                // let a = res.get();
                let a = res;
                view! {
                    <h3>"Server resource:"</h3>
                    <p>{a}</p>
                }
            })}
            </Suspense>
        </MainLayout>
    }
}

#[component]
pub fn Sandbox3() -> impl IntoView {
    // Shows how app behaves when request returns error
    let (resource_counter, resource_counter_set) = signal(0);
    let res = Resource::new(move || {resource_counter.get()},
                            |_| async move {get_error_from_server().await}
                           );
    let _res_value = move || {
        res.get()
            .map(
                |value| {leptos::logging::log!("Server value:  {value:?}");
                         format!("Server returned: {value:?}")}
                ).unwrap_or_else(|| "Loading".into())
    };

    let flowbite_button = "text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none";
    view! {
        <MainLayout>
            <button class=flowbite_button
                on:click=move |_| {
                    *resource_counter_set.write() +=1;
                    let counter_value = resource_counter.get();
                    leptos::logging::log!("Clicked sandbox 1, counter: {counter_value}");
                    
                } >
                Sandbox 1
            </button>

            <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || Suspend::new(async move {
                let a = res;
                view! {
                    <h3>"Server resource:"</h3>
                    <p>{a}</p>
                }
            })}
            </Suspense>
        </MainLayout>
    }
}