use crate::utils::*;

/// This algorithm takes all liked toppings and adds them to the pizza
pub fn all(_client: &[Client], addable: &Ing) -> Ing {
    addable.to_owned()
}
