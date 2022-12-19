use strum::EnumCount;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter as EnumIterMacro, EnumString};
use yew::prelude::*;

#[derive(Display, EnumCountMacro, EnumString)]
pub enum CheckBoxName {
    Scales,
    Horns,
}

#[derive(Display, EnumIterMacro, EnumString, Debug, PartialEq, Eq, Clone)]
pub enum SelectName {
    #[strum(serialize = "--Please choose an option--")]
    None,
    Dog,
    Cat,
    Hamster,
    Parrot,
    Spider,
    Goldfish,
}

#[derive(Display, EnumIterMacro, EnumCountMacro, EnumString, Debug, PartialEq, Eq, Clone)]
pub enum RadioName {
    Huey,
    Dewey,
    Louie,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MultiInputValue {
    pub checkbox: Vec<bool>,
    pub text: AttrValue,
    pub select: SelectName,
    pub radio: RadioName,
}

impl Default for MultiInputValue {
    fn default() -> Self {
        Self {
            checkbox: vec![false; CheckBoxName::COUNT],
            text: AttrValue::default(),
            select: SelectName::None,
            radio: RadioName::Huey,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MultiInputProps {
    pub multi_input_value: MultiInputValue,
    pub onsubmit: Callback<MultiInputValue>,
}
