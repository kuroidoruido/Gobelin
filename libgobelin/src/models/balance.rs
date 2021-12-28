use crate::exact_float::ExactFloat;

#[derive(Clone, Debug, PartialEq)]
pub struct Balance {
    pub name: String,
    pub amount: ExactFloat,
}
