use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(InputTextV3)]
pub fn input_text_v3() -> Html {
    let input_node_ref = use_node_ref();
    let text_state = use_state(|| AttrValue::from("".to_string()));

    let onchange = {
        let text_state = text_state.clone();
        let input_node_ref = input_node_ref.clone();
        Callback::from(move |_| {
            let input_element = input_node_ref.cast::<HtmlInputElement>();
            if let Some(input_element) = input_element {
                text_state.set(AttrValue::from(input_element.value()));
            }
        })
    };

    html! {
        <div>
            <label>
                {"Input Text V3"}
                <input ref={input_node_ref} value={&*text_state} {onchange}/>
            </label>
            <span>{&*text_state}</span>
        </div>
    }
}
