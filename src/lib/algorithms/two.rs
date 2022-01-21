use rayon::prelude::*;
use std::collections::HashSet;

use super::super::*;

pub fn two_client(clients: &[Client], addable: &Ing, _removeable: &Ing) -> Ing {
    clients
        .par_iter()
        .flat_map(|a| {
            clients.par_iter().map(|b| {
                let mut disliked = a.dislikes.clone();
                disliked.extend(b.dislikes.clone());
                let pizza = addable
                    .difference(&disliked)
                    .cloned()
                    .collect::<HashSet<String>>();
                (score(clients, &pizza), pizza)
            })
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or_default()
        .1
}
