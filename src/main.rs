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
            exit(1)
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
                //e unwrap per ritornare il valore dentro de Ok o smettere il programma 
                
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

            for product in &products{
                // TODO
                // if id.parse().unwrap() as u8 == product.id{
                //}
            }


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
