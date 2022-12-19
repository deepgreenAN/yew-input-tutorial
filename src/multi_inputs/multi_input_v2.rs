use crate::multi_inputs::{CheckBoxName, MultiInputProps, MultiInputValue, RadioName, SelectName};
use strum::{EnumCount, IntoEnumIterator};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[function_component(MultiInputV2)]
pub fn multi_input_v2(multi_input_props: &MultiInputProps) -> Html {
    let multi_input_value = &multi_input_props.multi_input_value;

    let checkbox_refs: [NodeRef; CheckBoxName::COUNT] = [use_node_ref(), use_node_ref()];
    let text_ref = use_node_ref();
    let select_ref = use_node_ref();
    let radio_refs: [NodeRef; RadioName::COUNT] = Default::default();

    let onsubmit = {
        let parent_onsubmit = multi_input_props.onsubmit.clone();
        let checkbox_refs = checkbox_refs.clone();
        let text_ref = text_ref.clone();
        let select_ref = select_ref.clone();
        let radio_refs = radio_refs.clone();

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

            let select: SelectName = select_ref
                .cast::<HtmlSelectElement>()
                .expect("This NodeRef should cast into HtmlSelectElement")
                .value()
                .parse()
                .expect("This Value should parse into SelectName");

            let radio: RadioName = radio_refs
                .iter()
                .find_map(|node_ref| {
                    let input_element = node_ref
                        .cast::<HtmlInputElement>()
                        .expect("This NodeRef should cast into HtmlInputElement");
                    if input_element.checked() {
                        Some(input_element.value())
                    } else {
                        None
                    }
                })
                .expect("All radio are not checked")
                .parse()
                .expect("This Value should parse into RadioName");

            let value = MultiInputValue {
                checkbox,
                text,
                select,
                radio,
            };
            parent_onsubmit.emit(value);
        })
    };

    html! {
        <fieldset class="form">
            <legend>{"Multi Input V2"}</legend>
            <div>
                <label>
                    {"Name:"}
                    <input
                        type="text"
                        ref={text_ref}
                        value={multi_input_value.text.clone()}
                    />
                </label>
            </div>
            <div>
                <label>
                    {CheckBoxName::Scales.to_string()}
                    <input
                        type="checkbox"
                        ref={checkbox_refs[CheckBoxName::Scales as usize].clone()}
                        checked={multi_input_value.checkbox[CheckBoxName::Scales as usize]}
                        value={CheckBoxName::Scales.to_string()}
                    />
                </label>
                <label>
                    {CheckBoxName::Horns.to_string()}
                    <input
                        type="checkbox"
                        ref={checkbox_refs[CheckBoxName::Horns as usize].clone()}
                        checked={multi_input_value.checkbox[CheckBoxName::Horns as usize]}
                        value={CheckBoxName::Horns.to_string()}
                    />
                </label>
            </div>
            <div>
                <label>
                    {"Choose a pet:"}
                    <select ref={select_ref} value={multi_input_value.select.to_string()}>
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
                                    name="v2_drone"
                                    ref={radio_refs[radio_name as usize].clone()}
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
