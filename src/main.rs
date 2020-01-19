use std::io;
use std::process::exit;


fn main() {
    #[derive(Debug)]
    struct Product{
        id: u8,
        name: String,
        quantity: u32,
        price: f32
    }

    // impl std::fmt::Display for Product{
    //     fn fmt(&self, &mut std::fmt::Formatter<'_>)->std::result::Result<(), std::fmt::Error>{
    //         format!("Prodotto: {}, Quantitá: {}, Prezzo {} €.", self.name, self.quantity, self.price);
    //     }
    // }
    
    // boh
    // fn get_products()->Vec<Product>{
    //     products
    // }
    // let mut products: Vec<Product> = get_products();

    let mut next_id:u8 = 3;
    let mut products = vec![
        Product{id: 1, name: String::from("Penna"), quantity: 20 , price: 1.0},
        Product{id: 2, name: String::from("Notebook"), quantity: 5 , price: 250.0}
    ];
    
    fn recieve_input()->String{
        let mut raw_string = String::new();
        io::stdin().read_line(&mut raw_string).expect("Error");
        raw_string.trim().to_string()
    }

    loop{

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
            
            let price:f32 = {
            
                let raw_price = recieve_input();
                
                // transformando String in f32 usando 'parse' per ritornare un valore o Ok(Value) o Err
                // e unwrap per ritornare il valore dentro de Ok o smettere il programma 
                
                raw_price.parse().expect("Per favore inserisca un numero")
            };

            println!("Quantitá del prodotto");
            
            let quantity: u32 = {
            
                let raw_quantity = recieve_input();
                raw_quantity.parse().expect("Per favore inserisca un numero")
            
            };

            products.push(Product{id:next_id, name, quantity, price});
            next_id+=1;

        }else if input == "2"{
            
            // printare i prodotti per scegliere quale editare

            for product in &products {
                println!("{:?}",product);
            }

            println!("-------------------------------------------------------------------");
            println!("Scegli l'id del prodotto che vuoi editare usando il'id del prodotto");
            println!("-------------------------------------------------------------------");
            
            let id: u8 = {
                let raw_id = recieve_input();
                raw_id.parse().expect("Per favore inserisca un numero")
            };
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
                            let price: f32 = {
                                let raw_price = recieve_input();
                                raw_price.parse().expect("Per favore inserisca un numero")
                            };

                            println!("Quale é la nuova quantitá del prodotto \"{}\"? attuale: {}", product.name, product.quantity);
                            let quantity: u32 = {
                                let raw_quantity = recieve_input();
                                raw_quantity.parse().expect("Per favore inserisca un numero")
                            };
                            
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

            products.remove(remove_index);
            println!("Prodotto rimosso!");

        }else if input == "3"{
            
            //TODO
            // fare la simulazione
        
        }else if input == "4"{
            for product in &products {
                println!("{:?}", product);
            }
        }else{
           continue 
        }

        
        
    }

}
