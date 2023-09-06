use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    log::debug!("Hello Console!");
    view! { cx, <h1>"Hello World!"</h1> }
}
