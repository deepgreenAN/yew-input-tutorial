use crate::multi_inputs::{CheckBoxName, MultiInputProps, MultiInputValue, RadioName, SelectName};
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[function_component(MultiInputV1)]
pub fn multi_input_v1(multi_input_props: &MultiInputProps) -> Html {
    let multi_input_value = &multi_input_props.multi_input_value;

    let checkbox_handle = use_mut_ref(|| multi_input_value.checkbox.clone());
    let text_handle = use_mut_ref(|| multi_input_value.text.clone());
    let select_handle = use_mut_ref(|| multi_input_value.select.clone());
    let radio_handle = use_mut_ref(|| multi_input_value.radio.clone());

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

    let select_onchange = {
        let select_handle = select_handle.clone();
        Callback::from(move |e: Event| {
            let select_element = e
                .target()
                .expect("Event should have a target when dispatched")
                .dyn_into::<HtmlSelectElement>()
                .expect("This EventTarget should cast into HtmlSelectElement");

            let selecte_name: SelectName = select_element
                .value()
                .parse()
                .expect("This Value should parse into SelectName");

            *select_handle.borrow_mut() = selecte_name;
        })
    };

    let radio_onchange = {
        let radio_handle = radio_handle.clone();
        Callback::from(move |e: Event| {
            let input_element = e
                .target()
                .expect("Event should have a target when dispatched")
                .dyn_into::<HtmlInputElement>()
                .expect("This EventTarget should cast into HtmlInputElement");

            if input_element.checked() {
                let radio_name: RadioName = input_element
                    .value()
                    .parse()
                    .expect("This Value should parse into RadioName");

                *radio_handle.borrow_mut() = radio_name;
            }
        })
    };

    let onsubmit = {
        let parent_onsubmit = multi_input_props.onsubmit.clone();
        let checkbox_handle = checkbox_handle.clone();
        let text_handle = text_handle.clone();
        let select_handle = select_handle.clone();
        let radio_handle = radio_handle.clone();

        Callback::from(move |_: MouseEvent| {
            let multi_input_value = MultiInputValue {
                checkbox: checkbox_handle.replace_with(|value| value.clone()),
                text: text_handle.replace_with(|value| value.clone()),
                select: select_handle.replace_with(|value| value.clone()),
                radio: radio_handle.replace_with(|value| value.clone()),
            };
            parent_onsubmit.emit(multi_input_value);
        })
    };

    html! {
        <fieldset class="form">
            <legend>{"Multi Input V1"}</legend>
            <div>
                <label>
                    {"Name:"}
                    <input
                        type="text"
                        onchange={text_onchange}
                        value={multi_input_value.text.clone()}
                    />
                </label>
            </div>
            <div>
                <label>
                    {CheckBoxName::Scales.to_string()}
                    <input
                        type="checkbox"
                        onchange={checkbox_onchange.clone()}
                        checked={multi_input_value.checkbox[CheckBoxName::Scales as usize]}
                        value={CheckBoxName::Scales.to_string()}
                    />
                </label>
                <label>
                    {CheckBoxName::Horns.to_string()}
                    <input
                        type="checkbox"
                        onchange={checkbox_onchange.clone()}
                        checked={multi_input_value.checkbox[CheckBoxName::Horns as usize]}
                        value={CheckBoxName::Horns.to_string()}
                    />
                </label>
            </div>
            <div>
                <label>
                    {"Choose a pet:"}
                    <select onchange={select_onchange} value={multi_input_value.select.to_string()}>
                        {
                            for SelectName::iter().map(|select_name|{
                                html_nested!{
                                    <option value={select_name.to_string()} selected={multi_input_value.select==select_name}>
                                        {select_name.to_string()}
                                    </option>
                                }
                            })
                        }
                    </select>
                </label>
            </div>
            <div>
                {
                    for RadioName::iter().map(|radio_name|{
                        html_nested!{
                            <label>
                                {radio_name.to_string()}
                                <input
                                    type="radio"
                                    name="v1_drone"
                                    onchange={radio_onchange.clone()}
                                    value={radio_name.to_string()}
                                    checked={multi_input_value.radio==radio_name}
                                />
                            </label>
                        }
                    })
                }
            </div>
            <div>
                <button onclick={onsubmit}>{"更新"}</button>
            </div>
        </fieldset>
    }
}
