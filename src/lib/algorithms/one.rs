use std::collections::HashSet;

use super::super::*;

/// This algorithm removes each topping and returns the pizza with the best performance
pub fn one_ingredient(clients: &[Client], addable: &Ing) -> Ing {
    let mut best: (usize, Ing) = (0, HashSet::default());
    for i in addable {
        let mut addable = addable.clone();
        addable.remove(i);
        let s = score(clients, &addable);
        if s > best.0 {
            best.0 = s;
            best.1 = addable;
        }
    }
    best.1
}

/// This algorithm removes one client's dislikes and returns the pizza with the best performance
pub fn one_client(clients: &[Client], addable: &Ing) -> Ing {
    let mut best: (usize, Ing) = (0, HashSet::default());
    for i in addable {
        let mut addable = addable.clone();
        addable.remove(i);
        let s = score(clients, &addable);
        if s > best.0 {
            best.0 = s;
            best.1 = addable;
        }
    }
    best.1
}
