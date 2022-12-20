mod double_inputs;
mod input_texts;
mod multi_inputs;

use crate::double_inputs::{DoubleInputV1, DoubleInputV2};
use crate::double_inputs::{DoubleInputValue, DouubleInputProps};
use crate::input_texts::{InputTextV1, InputTextV2, InputTextV3};
use crate::multi_inputs::{MultiInputProps, MultiInputValue};
use crate::multi_inputs::{MultiInputV1, MultiInputV2};

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let multi_input_value_handle = use_state(MultiInputValue::default);
    let double_input_value_handle = use_state(DoubleInputValue::default);

    let double_input_onsubmit = {
        let double_input_value_handle = double_input_value_handle.clone();
        Callback::from(move |double_input_value| {
            double_input_value_handle.set(double_input_value);
        })
    };
    let double_input_props = DouubleInputProps {
        double_input_value: (*double_input_value_handle).clone(),
        onsubmit: double_input_onsubmit,
    };

    let multi_input_onsubmit = {
        let multi_input_value_handle = multi_input_value_handle.clone();
        Callback::from(move |multi_input_value: MultiInputValue| {
            multi_input_value_handle.set(multi_input_value);
        })
    };
    let multi_input_props = MultiInputProps {
        multi_input_value: (*multi_input_value_handle).clone(),
        onsubmit: multi_input_onsubmit,
    };

    html! {
        <>
        <InputTextV1/>
        <InputTextV2/>
        <InputTextV3/>
        <DoubleInputV1 ..double_input_props.clone()/>
        <DoubleInputV2 ..double_input_props/>
        <div>{format!("{:?}", *double_input_value_handle)}</div>
        <MultiInputV1 ..multi_input_props.clone()/>
        <MultiInputV2 ..multi_input_props/>
        <div>{format!("{:?}", *multi_input_value_handle)}</div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
