mod input_texts;
mod multi_inputs;

use crate::input_texts::{InputTextV1, InputTextV2, InputTextV3};
use crate::multi_inputs::{MultiInputProps, MultiInputValue};
use crate::multi_inputs::{MultiInputV1, MultiInputV2};

use yew::prelude::*;
use yew::props;

#[function_component(App)]
fn app() -> Html {
    let multi_input_value_handle = use_state(MultiInputValue::default);
    let onsubmit = {
        let multi_input_value_handle = multi_input_value_handle.clone();
        Callback::from(move |multi_input_value: MultiInputValue| {
            multi_input_value_handle.set(multi_input_value);
        })
    };

    let multi_input_props = props!(MultiInputProps {
        multi_input_value: (*multi_input_value_handle).clone(),
        onsubmit: onsubmit
    });

    html! {
        <>
        <InputTextV1/>
        <InputTextV2/>
        <InputTextV3/>
        <MultiInputV1 ..multi_input_props.clone()/>
        <MultiInputV2 ..multi_input_props.clone()/>
        <div>{format!("{:?}", *multi_input_value_handle)}</div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
