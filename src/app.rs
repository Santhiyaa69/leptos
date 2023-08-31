use leptos::html::Form;
use leptos::*;
use leptos_router::FromFormData;
use serde::{Deserialize, Serialize};

use crate::{contact_form::ContactForm, error_handling::NumericInput, product::Product};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    user: String,
    pass: String,
}

#[component]
pub fn Todo<F>(cx: Scope, title: &'static str, on_click: F) -> impl IntoView
where
    F: Fn(String) + 'static + Clone,
{
    log!("Loaded on Init");

    let counter = create_rw_signal(cx, 0);

    //let title = "This is todo from var";

    let on_ev = move |_| {
        let c = counter.get() + 1;
        counter.set(c);
        on_click(c.to_string());
    };

    let dr = move || counter.get() + 45;

    let form_ref = create_node_ref::<Form>(cx);

    let on_submit = move |_| {
        let f = &form_ref.get().unwrap();
        let data = web_sys::FormData::new_with_form(f).unwrap();
        let fd = Data::from_form_data(&data).unwrap();
        log!("{}", serde_json::to_string(&fd).unwrap());
        f.reset();
    };

    view! { cx,
        <h5 on:click=on_ev>{dr}</h5>

        <form _ref=form_ref>
            <input type="text" name="user"/>
            <input type="text" name="pass"/>
            <button type="button" on:click=on_submit>
                "Save"
            </button>
        </form>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let on_todo = move |data: String| {
        log!("{data}");
    };

    let inc = move |data: i32| log!("{}", data);

    view! {
        cx,
        <div>
        // <span>"This is app"</span>
        // <span>"This is app2"</span>
        </div>
        // <Todo title="This is title from App" on_click=on_todo/>

        // <Product product_name="vicks".to_string() unit="PCS" on_click=inc/>

        // <ContactForm/>

        <NumericInput />
    }
}
