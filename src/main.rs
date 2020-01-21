use std::io;
use std::process::exit;

fn main() {
    
    enum ProductField{ 
            Id(u8),
            Name(String),
            Quantity(u32),
            Price(f32)
    };

    #[derive(Debug, Clone)]
    struct Product {
        id: u8,
        name: String,
        quantity: u32,
        price: f32
    }

    impl Product{
        fn modify(&mut self, field: &ProductField) -> bool{
            match field{
               ProductField::Id(val)        => self.id = *val,
               ProductField::Name(val)      => self.name = val.clone(),
               ProductField::Quantity(val)  => self.quantity = *val,
               ProductField::Price(val)     => self.price = *val,
            }
            true
        }
    } 


    // non chiedermi cosa succede qui.
    impl std::fmt::Display for Product{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->std::result::Result<(), std::fmt::Error>{
            write!(f, "Prodotto: {}, Quantitá: {}, Prezzo {} €.", self.name, self.quantity, self.price)
        }
    }
    
    struct ProductList{
        prods: Vec<Product>,
        next_id: u8
    }

    impl ProductList{
        
        fn print_all(&self){
            self.prods.iter().for_each(|p| println!("{}",p));
        }

        fn insert_prod(&mut self, prod: Product){
            self.prods.push(prod);
            self.next_id +=1;
        }

        fn get_prod(&self, id:u8) -> Option<&Product> {
            self.prods
                .iter()
                .filter(|p| p.id == id)
                .nth(0)
        }

        fn modify_prod(&mut self, id: u8, field: &ProductField) -> bool{
            match self.get_prod(id){
                Some(prod) => prod.modify(field),
                None => false
            } 
        }

        fn remove_prod(&mut self, id: u8) -> bool{
            match self.get_prod(id){
                Some(prod) => {self.prods.remove(prod.id as usize); true},
                None => false
            }
        }
    }

    
    // TODO connessione con banco di dati
    fn get_products() -> Vec<Product>{
        vec![
            Product{id: 1, name: String::from("Penna"), quantity: 20 , price: 1.0},
            Product{id: 2, name: String::from("Notebook"), quantity: 5 , price: 250.0}
        ]

    }

    let mut products = ProductList{
        prods: get_products(),
        next_id: get_products().last().unwrap().id
    };
    
    fn recieve_input() -> String {
        let mut raw_string = String::new();
        io::stdin().read_line(&mut raw_string).expect("Error reading string stream");
        raw_string.trim().to_string()
    }

    macro_rules! trim_parse{
        ()=>{
            {
                let raw_price = recieve_input();
                
                // transformando String in f32 usando 'parse' per ritornare un valore o Ok(Value) o Err
                // e unwrap per ritornare il valore dentro de Ok o smettere il programma 
                
                raw_price
                    .parse()
                    .expect("Per favore inserisca un numero")
        
            }
        }
    }


    loop{
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
        
        if input == "0"{
            exit(0)
        }

        println!("\n\n\n");

        if input == "1"{
            
            // ricevere i dati per creare un nuovo prodotto
            
            println!("Nome del prodotto");
            
            let name = recieve_input();

            println!("Prezzo del prodotto");
            
            let price:f32 = trim_parse!();

            println!("Quantitá del prodotto");
            
            let quantity: u32 = trim_parse!();

            products.insert_prod(Product{
                id:products.next_id,
                name, 
                quantity,
                price
            });

        }else if input == "2"{
            
            // printare i prodotti per scegliere quale editare

            print_products!();

            println!("-------------------------------------------------------------------");
            println!("Scegli l'id del prodotto che vuoi editare usando il'id del prodotto");
            println!("-------------------------------------------------------------------");
            
            let id: u8 = trim_parse!();
            
            let mut index = 0;
            let mut remove_index = 0;
            
            for product in &mut products{
                // TODO
                if product.id == id{
                    loop{
                        println!("---------------------------------");
                        println!("(1) Per editare il prodotto      ");
                        println!("(2) Per rimuovere il prodotto    ");
                        println!("(0) Per tornare al menu anteriore");
                        println!("---------------------------------");
                        
                        let input = recieve_input();
                        
                        if input == "0"{
                            break;
                        }else if input == "1"{
                            println!("Quale é il nuovo nome del prodotto \"{}\"?", product.name);
                            let name = recieve_input();
                            
                            println!("Quale é il nuovo prezzo del prodotto \"{}\"? attuale: {}", product.name, product.price);
                            let price: f32 = trim_parse!();

                            println!("Quale é la nuova quantitá del prodotto \"{}\"? attuale: {}", product.name, product.quantity);
                            let quantity: u32 = trim_parse!();
                            
                            *product = Product{
                                id: product.id,
                                name,
                                price,
                                quantity
                            };
                            println!("Prodotto aggiunto!");
                            break;
                        }else if input=="2"{
                            //rimuovere il prodotto
                            
                            remove_index = index;

                            break; 
                        }else{
                            continue;
                        }
                    }

                }else{
                    println!("----------------------");
                    println!("Scegli un id esistente");
                    println!("----------------------");
                }
            
                index+=1;
            }

            products.remove_prod(remove_index);
            println!("Prodotto rimosso!");

        }else if input == "3"{
            
            let mut products_simulation = ProductList{
                prods: vec![],
                // TODO non só cosa fare qua
                // valore inutile in questa variabile
                next_id: 0
            };

            loop{
                
                println!("-----------------------------------------------------");
                println!("(1) Per aggiungere un prodotto");
                println!("(2) Per rimuovere un prodotto");
                println!("(0) Per finire la simulazione e fare lo \"scontrino\"");
                println!("-----------------------------------------------------");
            
                let input = recieve_input();
                
                if input == "0"{
                    //TODO finire la simulazione e fare lo "scontrino"
                    products_simulation.print_all();

                    println!("subtotale: {}",
                             products_simulation
                                .iter()
                                .fold(0.0, |acc,i|acc + (i.price * (i.quantity as f32))));
                    
                    break;
                }else if input == "1"{
                
                    print_products!();
                    
                    println!("Scegli l'id del prodotto");
                    
                    let id: u8 = trim_parse!();

                    for product in &mut products{
                        if id == product.id {
                            loop{
                                println!("quanti prodotti del tipo \"{}\" vuoi aggiungere alla lista?", product.name);
                                
                                let quantity: u32 = trim_parse!();
                                
                                if product.quantity >= quantity && quantity > 0{
                                    
                                    
                                    match products_simulation.get_prod(id){
                                        Some(prod)  => (),  //TODO modificare il prodotto
                                        None        => ()   // TODO aggiungere il prodotto products_simulation.insert_prod(Product{products.get_prod(id).clone()})
                                    }

                                    product.quantity -= quantity;
                                    break;
                                }else{
                                    println!("Quantitá eccessiva a quella nello stock");
                                }
                            }
                        }
                    }

                }else if input == "2"{
                    
                    // TODO rimuovere un prodotto dalla lista
                    // e aggiungerlo allo stock
                    
                    products_simulation.print_all();
                    
                    println!("Scegli l'id del prodotto");
                    
                    let id: u8 = trim_parse!();
                    
                    let mut index = 0;
                    let mut can_remove = false;
                    let mut remove_index = 0;
                    for product in &mut products_simulation{
                        if id == product.id {
                        
                            println!("quanti prodotti del tipo \"{}\" vuoi rimuovere alla lista?", product.name);
                            
                            let quantity: u32 = trim_parse!();
                            
                            if product.quantity > quantity && quantity > 0{
                                
                                for item in &mut products{
                                    if id == item.id{
                                        item.quantity += quantity;
                                    }
                                }
                                
                                product.quantity -= quantity;
                                break;
                            }else if product.quantity == quantity {
                                can_remove = true;
                                remove_index = index; 
                            }else{
                                println!("Quantitá eccessiva a quella nello scontrino o > 0");
                            }
                        }
                        index+=1;
                    }
                    if can_remove{
                        products_simulation.remove_prod(remove_index);
                    }

                }else{
                    continue;
                }
            }

        }else if input == "4"{

            print_products!();
        
        }else{
           continue 
        }
    }
}
