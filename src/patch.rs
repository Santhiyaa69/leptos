use leptos::{*, html::Form};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

pub fn patch(form: &HtmlElement<Form>, data: impl serde::Serialize) {
    let qs = serde_qs::to_string(&data).unwrap();
    let url_encode = serde_urlencoded::from_str::<Vec<(String, String)>>(&qs).unwrap();
        form.reset();
        let el = form.elements();
            for num in 0..el.length() {
                for i in &url_encode {
                  if let Some(d)  = el.named_item(&i.0) {
                    let e =  JsValue::from(d); 
                    let x = HtmlInputElement::from(e);
                    x.set_value(&i.1);
                  }
                }
            }
}