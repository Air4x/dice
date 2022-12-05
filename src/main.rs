use rand::prelude::*;
use std::env;
use std::collections::VecDeque;

enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

fn main() {
    // Creazione rng_handler per la creazione di
    // numeri randomici
    let mut rng_handler = rand::thread_rng();
    let mut dadi: Vec<Dice> = Vec::new();
    let mut lanci: Vec<i32> = Vec::new();

    // ottenimento dadi da linea di comando
    // rimuovendo il primo argomento che
    // è il nome con cui è stato chiamato
    // l'eseguibile
    let mut arguments: VecDeque<String> = env::args().collect();
    arguments.pop_front();
    for arg in arguments {
        dadi.push(to_dice(arg));
    }
    // ottenimento risultati dei dadi
    for dado in dadi {
        lanci.push(dado.lancio(&mut rng_handler));
    }
    // stampa dei lanci
    for risultato in lanci {
        print!("{}\t", risultato);
    }
}

fn to_dice(dado: String) -> Dice {
    match dado.as_str() {
        "d4" => Dice::D4,
        "d6" => Dice::D6,
        "d8" => Dice::D8,
        "d10" => Dice::D10,
        "d12" => Dice::D12,
        "d20" => Dice::D20,
        _ => Dice::D4,
    }
}

impl Dice {
    fn lancio(&self, handle: &mut ThreadRng) -> i32 {
        match *self {
            Dice::D4 => handle.gen_range(1..=4),
            Dice::D6 => handle.gen_range(1..=6),
            Dice::D8 => handle.gen_range(1..=8),
            Dice::D10 => handle.gen_range(1..=10),
            Dice::D12 => handle.gen_range(1..=12),
            Dice::D20 => handle.gen_range(1..=20),
        }
    }
}
