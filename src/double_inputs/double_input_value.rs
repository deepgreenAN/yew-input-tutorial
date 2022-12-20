use strum::EnumCount;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter as EnumIterMacro, EnumString};
use yew::prelude::*;

#[derive(Display, EnumCountMacro, EnumString, EnumIterMacro, Clone, Copy)]
pub enum CheckBoxName {
    Scales,
    Horns,
}

/// 親子コンポーネント間で渡し合うデータ
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DoubleInputValue {
    pub text: AttrValue,
    pub checkbox: Vec<bool>,
}

impl Default for DoubleInputValue {
    fn default() -> Self {
        Self {
            text: AttrValue::default(),
            checkbox: {
                let mut bool_vec = vec![false; CheckBoxName::COUNT];
                bool_vec[CheckBoxName::Scales as usize] = true;
                bool_vec
            },
        }
    }
}

/// props
#[derive(Properties, PartialEq, Clone)]
pub struct DouubleInputProps {
    pub double_input_value: DoubleInputValue,
    pub onsubmit: Callback<DoubleInputValue>,
}
