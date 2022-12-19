use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(InputTextV1)]
pub fn input_text_v1() -> Html {
    let text_state = use_state(|| AttrValue::from("".to_string()));

    let onchange = {
        let text_state = text_state.clone();
        Callback::from(move |e: Event| {
            let target = e.target();
            let input_element = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input_element) = input_element {
                text_state.set(AttrValue::from(input_element.value()));
            }
        })
    };

    html! {
        <div>
            <label>
                {"Input Text V1"}
                <input value={&*text_state} {onchange}/>
            </label>
            <span>{&*text_state}</span>
        </div>
    }
}
