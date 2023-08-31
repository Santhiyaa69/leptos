use leptos::{*, html::Form};
use leptos_router::FromFormData;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug)]
struct Product {
  id: usize,
  name: &'static str,
  price: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    inv: String,
    qty: f64,
    price: f64
}

#[component]
pub fn Product<F>(cx:Scope, product_name:String, unit: & 'static str, on_click: F ) -> impl IntoView
where 
    F: Fn(i32) + 'static + Clone
{
    log!("product component");
    // let product_name = "abc";
    let count = create_rw_signal(cx, 0);
    
    let p1 = Product{
        id: 1,
        name: "vicks" ,
        price: 2
    };
    let p2 = Product{
        id: 2,
        name: "halls",
        price: 1
    };
    let products =  move || {vec![p1.clone(),p2.clone()]};
  
   
    let inc = move |_| {
        let c  = count.get() + 1;
        count.set(c);
        on_click(c)
    };
    // let counter = move||{
    //     count.get()
    // };

    view! {
        cx,
        <div>
        <label>"Inventory :" {product_name}</label>
        <label>"Unit :" {unit}</label>
        </div>
       
        <div>
        <Show
            when={move || count.get() > 0}
            fallback=|_| ()
        >
            {count}
        </Show>
        </div>

        <div>
        <button on:click=inc>"Click Me"</button>
        </div>

        <For
        each=products
        key=|product| product.id
        view=move |cx, product: Product| {
          view! {
            cx,
           <div>
           <label>"product name : "{move || product.name} "price : " {move || product.price}</label>
           </div>
          }
        }
      />
    }
}