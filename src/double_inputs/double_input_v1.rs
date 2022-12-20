use crate::double_inputs::{CheckBoxName, DoubleInputValue, DouubleInputProps};
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(DoubleInputV1)]
pub fn double_input_v1(double_input_props: &DouubleInputProps) -> Html {
    let double_input_value = &double_input_props.double_input_value;

    let checkbox_handle = use_mut_ref(|| double_input_value.checkbox.clone());
    let text_handle = use_mut_ref(|| double_input_value.text.clone());

    let checkbox_onchange = {
        let checkbox_handle = checkbox_handle.clone();
        Callback::from(move |e: Event| {
            let input_element = e
                .target()
                .expect("Event should have a target when dispatched")
                .dyn_into::<HtmlInputElement>()
                .expect("This EventTarget should cast into HtmlInputElement");

            let changed_checkbox_name: CheckBoxName = input_element
                .value()
                .parse()
                .expect("This Value should parse into CheckBoxName");

            checkbox_handle.borrow_mut()[changed_checkbox_name as usize] = input_element.checked();
        })
    };

    let text_onchange = {
        let text_handle = text_handle.clone();
        Callback::from(move |e: Event| {
            let input_element = e
                .target()
                .expect("Event should have a target when dispatched")
                .dyn_into::<HtmlInputElement>()
                .expect("This EventTarget should cast into HtmlInputElement");

            *text_handle.borrow_mut() = AttrValue::from(input_element.value());
        })
    };

    let onsubmit = {
        let parent_onsubmit = double_input_props.onsubmit.clone();
        let checkbox_handle = checkbox_handle;
        let text_handle = text_handle;

        Callback::from(move |_: MouseEvent| {
            let double_input_value = DoubleInputValue {
                checkbox: checkbox_handle.replace_with(|value| value.clone()),
                text: text_handle.replace_with(|value| value.clone()),
            };
            parent_onsubmit.emit(double_input_value);
        })
    };

    html! {
        <fieldset class="double-form">
            <legend>{"Double nput V1"}</legend>
            <div>
                {
                    for CheckBoxName::iter().map(|check_box_name|{
                        html_nested!{
                            <label>
                                {check_box_name.to_string()}
                                <input
                                    type="checkbox"
                                    onchange={checkbox_onchange.clone()}
                                    checked={double_input_value.checkbox[check_box_name as usize]}
                                    value={check_box_name.to_string()}
                                />
                            </label>
                        }
                    })
                }
            </div>
            <div>
                <label>
                    {"Name:"}
                    <input
                        type="text"
                        onchange={text_onchange}
                        value={double_input_value.text.clone()}
                    />
                </label>
            </div>
            <div>
                <button onclick={onsubmit}>{"更新"}</button>
            </div>
        </fieldset>
    }
}
