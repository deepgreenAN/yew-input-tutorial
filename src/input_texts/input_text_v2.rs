use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(InputTextV2)]
pub fn input_text_v2() -> Html {
    let text_state = use_state(|| AttrValue::from("".to_string()));

    let onchange = {
        let text_state = text_state.clone();
        Callback::from(move |e: Event| {
            let input_element = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input_element) = input_element {
                text_state.set(AttrValue::from(input_element.value()));
            }
        })
    };

    html! {
        <div>
            <label>
                {"Input Text V2"}
                <input value={&*text_state} {onchange}/>
            </label>
            <span>{&*text_state}</span>
        </div>
    }
}
