use leptos::*;

#[component]
pub fn NumericInput(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, Ok(0));

    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());

    view! { cx,
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|cx, errors| view! { cx,
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                                .collect_view(cx)
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
