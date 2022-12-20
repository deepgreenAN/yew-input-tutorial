use crate::double_inputs::{CheckBoxName, DoubleInputValue, DouubleInputProps};
use strum::{EnumCount, IntoEnumIterator};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(DoubleInputV2)]
pub fn double_input_v2(double_input_props: &DouubleInputProps) -> Html {
    let double_input_value = &double_input_props.double_input_value;

    let checkbox_refs: [NodeRef; CheckBoxName::COUNT] = [use_node_ref(), use_node_ref()];
    let text_ref = use_node_ref();

    let onsubmit = {
        let parent_onsubmit = double_input_props.onsubmit.clone();
        let checkbox_refs = checkbox_refs.clone();
        let text_ref = text_ref.clone();

        Callback::from(move |_| {
            let checkbox = checkbox_refs
                .iter()
                .map(|node_ref| {
                    node_ref
                        .cast::<HtmlInputElement>()
                        .expect("This NodeRef should cast into HtmlInputElement")
                        .checked()
                })
                .collect::<Vec<_>>();

            let text: AttrValue = text_ref
                .cast::<HtmlInputElement>()
                .expect("This NodeRef should cast into HtmlInputElement")
                .value()
                .into();

            let value = DoubleInputValue { checkbox, text };

            parent_onsubmit.emit(value);
        })
    };

    html! {
        <fieldset class="double-form">
            <legend>{"Double nput V2"}</legend>
            <div>
                { for CheckBoxName::iter().map(|check_box_name|{
                    html_nested!{
                        <label>
                            {check_box_name.to_string()}
                            <input
                                type="checkbox"
                                ref={checkbox_refs[check_box_name as usize].clone()}
                                checked={double_input_value.checkbox[check_box_name as usize]}
                                value={check_box_name.to_string()}
                            />
                        </label>
                    }
                })}
            </div>
            <div>
                <label>
                    {"Name:"}
                    <input
                        type="text"
                        ref={text_ref}
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
