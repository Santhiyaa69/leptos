use leptos::{*, html::{Form, Input}};
use leptos_router::FromFormData;
use serde::{Deserialize, Serialize};

use crate::patch;

// #[derive(Debug,Clone,Deserialize, Serialize)]
// pub struct Address {
//     pub address: String,
//     pub state: String,
// }


#[derive(Debug,Clone,Deserialize, Serialize)]
pub struct Contact {
    pub name: String,
    pub address: String,
    pub state: String,
}

#[component]
pub fn ContactForm(cx:Scope) -> impl IntoView {
    let form_ref = create_node_ref::<Form>(cx);
    let input_ref = create_node_ref::<Input>(cx);

    let submit = move |_| {
        let form = &form_ref.get().unwrap();
        let form_data = web_sys::FormData::new_with_form(&form).unwrap();      
        // let data = Contact::from_form_data(&form_data).unwrap();
        let data = Contact {
            name: "asddf".to_string(),
            address: "tuty".to_string(),
            state: "Tamilnadu".to_string()
        };
    
        let input = input_ref.get().unwrap();
        input.set_value("TN");
        // patch::patch(form,data);    
    };

    view! {
        cx,
        <form ref=form_ref>
            <div>
                <label style="font-weight:bold">"Name : "</label>
                <input type="text" name="name"/>
            </div>
            <div>
                <label style="font-weight:bold">"Address : "</label>
                <input type="text" name="address"/>
            </div>
            <div>
                <label style="font-weight:bold">"State : "</label>
                <input type="text" name="state" ref=input_ref/>
            </div>
            <div>
                <button type="button" on:click=submit>"Save"</button>
            </div>
        </form>
    }
}