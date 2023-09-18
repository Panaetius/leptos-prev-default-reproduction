use crate::error_template::{AppError, ErrorTemplate};
#[cfg(feature = "ssr")]
use axum::extract::Multipart;
#[cfg(feature = "ssr")]
use futures_util::stream::StreamExt;
use leptos::{ev::SubmitEvent, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (show, set_show) = create_signal(cx, false);
    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-prev-default-reproduction.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <nav>
                <button on:click=move |_| { set_show.update(|v| *v = !*v) }>Upload</button>
                <MyUploadForm show=show set_show=set_show />
            </nav>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    view! { cx,
        <div></div>
    }
}

#[cfg(feature = "ssr")]
pub async fn upload_file(mut multipart: Multipart) -> axum::http::StatusCode {
    while let Some(field) = multipart.next_field().await.unwrap() {
        println!("{:?}", field.name())
    }
    axum::http::StatusCode::ACCEPTED
}
#[component]
fn MyUploadForm(cx: Scope, show: ReadSignal<bool>, set_show: WriteSignal<bool>) -> impl IntoView {
    let on_submit = move |ev: SubmitEvent| {
        // uncomment this to not panic
        // ev.prevent_default();
        set_show(false);
    };
    view! {cx,
        <Show when=move || { show() } fallback=|_| { () }>

            <Form
                action="/api/upload_file"
                method="POST"
                enctype="multipart/form-data".to_string()
                // comment this for normal upload functionality
                on:submit=on_submit>
                <input type="file" name="file"/>
                <button type="submit">Submit</button>
            </Form>
        </Show>
    }
}
