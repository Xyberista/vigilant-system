use rayon::prelude::*;

use super::super::*;

/// This algorithm removes each topping and returns the pizza with the best performance
pub fn one_ingredient(clients: &[Client], addable: &Ing, _removeable: &Ing) -> Ing {
    addable
        .par_iter()
        .map(|i| {
            let mut pizza = addable.clone();
            pizza.remove(i);
            (score(clients, &pizza), pizza)
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .1
}

/// This algorithm removes one client's dislikes and returns the pizza with the best performance
pub fn one_client(clients: &[Client], addable: &Ing, _removeable: &Ing) -> Ing {
    clients
        .par_iter()
        .map(|c| {
            let pizza = addable.difference(&c.dislikes).cloned().collect();
            (score(clients, &pizza), pizza)
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .1
}

pub fn one_disliked(clients: &[Client], addable: &Ing, removeable: &Ing) -> Ing {
    removeable
        .par_iter()
        .map(|i| {
            let mut pizza = addable.clone();
            pizza.remove(i);
            (score(clients, &pizza), pizza)
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or_default()
        .1
}
