use std::io;
use std::process::exit;

fn main() {
    #[derive(Debug, Clone)]
    struct Product {
        id: u8,
        name: String,
        quantity: u32,
        price: f32,
    }

    impl Product {
        fn modify(&mut self, new: &Product) -> bool {
            *self = Product {
                id: self.id,
                ..new.clone()
            };
            true
        }

        fn modify_quantity(&mut self, quantity: u32) {
            self.quantity = quantity;
        }
    }

    // non chiedermi cosa succede qui.
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

        fn get_prod(&self, id: u8) -> Option<&Product> {
            self.prods.iter().filter(|p| p.id == id).nth(0)
        }

        fn get_prod_mut(&mut self, id: u8) -> Option<&mut Product>{
            self.prods.iter_mut().filter(|p| p.id == id).nth(0)
        }

        fn modify_prod(&mut self, id: u8, new: &Product) -> bool {
            match self.get_prod_mut(id) {
                Some(prod) => prod.modify(new),
                None => false,
            }
        }

        fn remove_prod(&mut self, id: u8) -> bool {
            match self.get_prod_mut(id) {
                Some(prod) => {
                    self.prods.remove(prod.id as usize);
                    true
                }
                None => false,
            }
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

    let mut products = ProductList {
        prods: get_products(),
        next_id: get_products().last().unwrap().id,
    };

    fn recieve_input() -> String {
        let mut raw_string = String::new();
        io::stdin()
            .read_line(&mut raw_string)
            .expect("Error reading string stream");
        raw_string.trim().to_string()
    }

    macro_rules! trim_parse {
        () => {{
            let raw_price = recieve_input();

            // transformando String in f32 usando 'parse' per ritornare un valore o Ok(Value) o Err
            // e unwrap per ritornare il valore dentro de Ok o smettere il programma

            raw_price.parse().expect("Per favore inserisca un numero")
        }};
    }

    loop {
        // TODO testare il programma

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

                match products.get_prod(id) {
                    //TODO
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
                                products.modify_prod(
                                    prod.id,
                                    &Product {
                                        id: prod.id,
                                        name,
                                        price,
                                        quantity,
                                    },
                                );

                                println!("Prodotto modificato!");
                                break;
                            } else if input == "2" {
                                //rimuovere il prodotto

                                products.remove_prod(prod.id);
                                println!("Prodotto rimosso!");

                                break;
                            } else {
                                continue;
                            }
                        }
                    }

                    None => println!("Id {} non incontrato", id),
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
                        "subtotale: {}",
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
                                        Some(prod_ref) => {
                                            prod_ref.modify_quantity(prod_ref.quantity + quantity)
                                        }
                                        None => {
                                            products_simulation.insert_prod(Product {
                                                quantity,
                                                ..stock_product.clone()
                                            });
                                        }
                                    }
                                    stock_product
                                        .modify_quantity(stock_product.quantity - quantity);
                                    break;
                                } else {
                                    println!("Quantitá eccessiva a quella nello stock");
                                }
                            }
                            None => println!("Prodotto con Id: {} non incontrato", id),
                        };
                    }
                } else if input == "2" {
                    // TODO rimuovere un prodotto dalla lista
                    // e aggiungerlo allo stock

                    products_simulation.print_all();

                    println!("Scegli l'id del prodotto");

                    let id: u8 = trim_parse!();

                    // TODO refactor point
                    loop {
                        match products_simulation.get_prod(id) {
                            Some(cart_product) => {
                                println!(
                                    "quanti prodotti del tipo \"{}\" vuoi rimuovere dalla lista?",
                                    cart_product.name
                                );

                                let quantity: u32 = trim_parse!();

                                if cart_product.quantity <= quantity && quantity >= 0 {
                                    let mut prod_ref = products.get_prod(id).unwrap();
                                    prod_ref.modify_quantity(prod_ref.quantity - quantity);

                                    break;
                                } else {
                                    println!("Quantitá eccessiva a quella nella simulazione");
                                }
                            }
                            None => println!("Prodotto con Id: {} non incontrato", id),
                        };
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
