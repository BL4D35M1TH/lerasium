use crate::functions::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/main.scss"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <nav>
                <ul>
                    <li><a href="/">"Home"</a></li>
                    <li><a href="/about">"About"</a></li>
                    <li><a href="/contact">"Contact Us"</a></li>
                    <li><a href="/lucky">"Lucky Number"</a></li>
                    <li><a href="/transition">"Transition"</a></li>
                </ul>
            </nav>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/about" view=|cx| view! { cx,
                        <Title text="About"/>
                        <Script>"console.log('Hello, world!');"</Script>
                        <p>"What about it, mate?"</p> }
                    />
                    <ContactUs/>
                    <Route path="/:random" view=|cx| {
                        let params = use_params_map(cx);
                        let uri_param: String = params()
                            .get("random")
                            .cloned()
                            .unwrap_or_default();
                        view! { cx, {uri_param} }
                    }/>
                    <Route path="/lucky" view=|cx| view! { cx, <LuckyNumber/>}/>
                    <Route path="/transition" view=|cx| view! { cx, <TransitionExample/>}/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component(transparent)]
pub fn ContactUs(cx: Scope) -> impl IntoView {
    log!("Re-rendered");
    view! { cx,
        <Route path="/contact" view=move |cx| {
            view! { cx, "Don't bother" }
        }/>
    }
}

#[component]
pub fn LuckyNumber(cx: Scope) -> impl IntoView {
    let default_server_fn_args: (u8, u8) = (46, 2);
    let (my_server_fn_args, set_my_server_fn_args) = create_signal(cx, default_server_fn_args);
    let server_fn_result = create_resource(cx, my_server_fn_args, |(x, y)| my_server_fn(x, y));
    view! { cx,
        <Suspense fallback=||"Loading...".to_string()>
            { move || server_fn_result.read() }
            "Loaded"
        </Suspense>
    }
}

#[component]
fn TransitionExample(cx: Scope) -> impl IntoView {
    let (list_size, set_list_size) = create_signal(cx, 1_u32);
    let list_items = create_resource(cx, list_size, pretend_external_list_items);
    let (is_list_updating, set_is_list_updating) = create_signal(cx, false);
    view! {cx,
        <p>"Total items: " {list_size}</p>
        <button on:click=move|_| set_list_size.set(list_size.get() + 1)>"Add an item"</button>
        <Show
            when=move || is_list_updating.get()
            fallback=|_| ""
        >
            <p><i>"Updating the list..."</i></p>
        </Show>
        <Transition
            fallback=move || view! { cx, <p>"Loading"</p> }
            set_pending=set_is_list_updating.into()
        >
            { view! { cx,
                <ul>
                    <For
                        each=move || list_items.read().unwrap_or_default()
                        key=move |item| item.clone()
                        view=|cx, item| view! { cx, <li>{item}</li>}
                    />
                </ul>
            }}
        </Transition>
    }
}

async fn pretend_external_list_items(count: u32) -> Vec<u32> {
    futures_timer::Delay::new(std::time::Duration::from_secs(1)).await;
    (1..=count).collect()
}
