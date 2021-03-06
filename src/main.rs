use std::io;
use std::process::exit;

#[derive(Debug, Clone)]
struct Product {
    id: u8,
    name: String,
    quantity: u32,
    price: f32,
}

impl Product {
    fn modify(&mut self, new: &Product) {
        *self = Product {
            id: self.id,
            ..new.clone()
        };
    }

    fn add(&mut self, quantity: u32) {
        self.quantity += quantity;
    }

    fn subtract(&mut self, quantity: u32) {
        self.quantity -= quantity;
    }
}

// For now, i don't understand this.
impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Id: {}, Prodotto: {}, Quantitá: {}, Prezzo {} €.",
            self.id, self.name, self.quantity, self.price
        )
    }
}

struct ProductList {
    prods: Vec<Product>,
    next_id: u8,
}

impl ProductList {
    fn print_all(&self) {
        self.prods.iter().for_each(|p| println!("{}", p));
    }

    fn insert_prod(&mut self, prod: Product) {
        self.prods.push(prod);
        self.next_id += 1;
    }

    fn get_prod(&mut self, id: u8) -> Option<&mut Product> {
        self.prods.iter_mut().filter(|p| p.id == id).nth(0)
    }

    fn get_index(&self, id: u8) -> Option<usize> {
        self.prods.iter().position(|p| p.id == id)
    }

    fn remove_prod(&mut self, id: u8) {
        self.prods.remove(self.get_index(id).unwrap());
    }

    fn get_all(&self) -> &Vec<Product> {
        &self.prods
    }
}

// TODO connessione con banco di dati
fn get_products() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: String::from("Penna"),
            quantity: 20,
            price: 1.0,
        },
        Product {
            id: 2,
            name: String::from("Notebook"),
            quantity: 5,
            price: 250.0,
        },
    ]
}

fn recieve_input() -> String {
    let mut raw_string = String::new();
    io::stdin()
        .read_line(&mut raw_string)
        .expect("Error reading string stream");
    raw_string.trim().to_string()
}

macro_rules! trim_parse {
    () => {{
        let raw_value = recieve_input();

        // transformando String in f32 usando 'parse' per ritornare un valore o Ok(Value) o Err
        // e unwrap per ritornare il valore dentro de Ok o smettere il programma

        raw_value.parse().expect("Per favore inserisca un numero")
    }};
}

fn main() {
    let mut products = ProductList {
        prods: get_products(),
        next_id: get_products().last().unwrap().id + 1,
    };

    loop {
        print!(
            "
-----------------------------------------------------
(1) per registrare un prodotto.
(2) per editare un prodotto.
(3) per emulare una compera e generare uno scontrino.
(4) per mostrare tutti i prodotti
(0 o Ctrl+C) per uscire.
-----------------------------------------------------
Scegli:
"
        );

        let input = recieve_input();

        if input == "0" {
            exit(0)
        }

        println!("\n\n\n");

        if input == "1" {
            // ricevere i dati per creare un nuovo prodotto

            println!("Nome del prodotto");

            let name = recieve_input();

            println!("Prezzo del prodotto");
            let price: f32 = trim_parse!();

            println!("Quantitá del prodotto");

            let quantity: u32 = trim_parse!();

            products.insert_prod(Product {
                id: products.next_id,
                name,
                quantity,
                price,
            });
        } else if input == "2" {
            // printare i prodotti per scegliere quale editare

            loop {
                products.print_all();
                println!("-----------------------------------------");
                println!("Scegli l'id del prodotto che vuoi editare");
                println!("-----------------------------------------");

                let id: u8 = trim_parse!();

                let mut can_remove = false;
                //TODO
                match products.get_prod(id) {
                    Some(prod) => {
                        loop {
                            println!("---------------------------------");
                            println!("(1) Per editare il prodotto      ");
                            println!("(2) Per rimuovere il prodotto    ");
                            println!("(0) Per tornare al menu anteriore");
                            println!("---------------------------------");

                            let input = recieve_input();

                            if input == "0" {
                                break;
                            } else if input == "1" {
                                println!("Quale é il nuovo nome del prodotto \"{}\"?", prod.name);
                                let name = recieve_input();

                                println!(
                                    "Quale é il nuovo prezzo del prodotto \"{}\"? attuale: {}",
                                    prod.name, prod.price
                                );
                                let price: f32 = trim_parse!();

                                println!(
                                    "Quale é la nuova quantitá del prodotto \"{}\"? attuale: {}",
                                    prod.name, prod.quantity
                                );
                                let quantity: u32 = trim_parse!();

                                // modify prod
                                // uso l'& per motivi di studio, non altro
                                prod.modify(&Product {
                                    id: prod.id,
                                    name,
                                    price,
                                    quantity,
                                });

                                println!("Prodotto modificato!");
                                break;
                            } else if input == "2" {
                                //rimuovere il prodotto
                                can_remove = true;

                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                    None => {
                        println!("Id {} non incontrato", id);
                    }
                }
                if can_remove {
                    products.remove_prod(id);
                    println!("Prodotto con Id:{} rimosso!", id);
                }
            }
        } else if input == "3" {
            let mut products_simulation = ProductList {
                prods: vec![],
                // TODO non só cosa fare qua
                // valore inutile in questa variabile
                next_id: 0,
            };
            loop {
                println!("-----------------------------------------------------");
                println!("(1) Per aggiungere un prodotto");
                println!("(2) Per rimuovere un prodotto");
                println!("(0) Per finire la simulazione e fare lo \"scontrino\"");
                println!("-----------------------------------------------------");

                let input = recieve_input();

                if input == "0" {
                    // finire la simulazione e fare lo "scontrino"
                    products_simulation.print_all();

                    println!(
                        "subtotale: {} €",
                        products_simulation
                            .get_all()
                            .iter()
                            .fold(0.0, |acc, i| acc + (i.price * (i.quantity as f32)))
                    );

                    break;
                } else if input == "1" {
                    products.print_all();

                    println!("Scegli l'id del prodotto");

                    let id: u8 = trim_parse!();

                    loop {
                        match products.get_prod(id) {
                            Some(stock_product) => {
                                println!(
                                    "quanti prodotti del tipo \"{}\" vuoi aggiungere alla lista?",
                                    stock_product.name
                                );

                                let quantity: u32 = trim_parse!();

                                if stock_product.quantity >= quantity && quantity > 0 {
                                    match products_simulation.get_prod(id) {
                                        Some(prod_ref) => prod_ref.add(quantity),
                                        None => {
                                            products_simulation.insert_prod(Product {
                                                quantity,
                                                ..stock_product.clone()
                                            });
                                        }
                                    }

                                    stock_product.subtract(quantity);
                                    break;
                                } else {
                                    println!("Quantitá eccessiva a quella nello stock");
                                }
                            }
                            None => {
                                println!("Prodotto con Id: {} non incontrato", id);
                                break;
                            }
                        }
                    }
                } else if input == "2" {
                    // TODO rimuovere un prodotto dalla lista
                    // e aggiungerlo allo stock

                    products_simulation.print_all();

                    println!("Scegli l'id del prodotto");

                    let id: u8 = trim_parse!();

                    // TODO refactor point
                    let mut can_remove = false;
                    loop {
                        match products_simulation.get_prod(id) {
                            Some(cart_product) => {
                                println!(
                                    "quanti prodotti del tipo \"{}\" vuoi rimuovere dalla lista?",
                                    cart_product.name
                                );

                                let quantity: u32 = trim_parse!();

                                if cart_product.quantity >= quantity {
                                    if cart_product.quantity == quantity {
                                        can_remove = true;
                                    }

                                    let prod_ref = products.get_prod(id).unwrap();

                                    prod_ref.add(quantity);
                                    cart_product.subtract(quantity);

                                    break;
                                } else {
                                    println!("Quantitá eccessiva a quella nella simulazione");
                                }
                            }
                            None => {
                                println!("Prodotto con Id: {} non incontrato", id);
                                break;
                            }
                        };
                    }
                    if can_remove {
                        products_simulation.remove_prod(id);

                        // println!("Produto con Id:{} rimosso.", id);
                    }
                } else {
                    continue;
                }
            }
        } else if input == "4" {
            products.print_all();
        } else {
            continue;
        }
    }
}
