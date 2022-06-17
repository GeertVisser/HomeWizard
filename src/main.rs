// Silence some warnings so they don't distract from the exercise.
//#![allow(dead_code, unused_variables)]

use p1_api::{ get_current_date, setup, gather_data };

fn main() {

//    let future = hello_world(); // Nothing is printed
//    block_on(future); // `future` is run and "hello, world!" is printed

    match get_current_date() {
        Ok(date) => println!("We've time travelled to {}!!", date),
        Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
    }

    match setup(){
        Ok(result) => println!("Setup succesfull"),
        Err(e) => eprintln!("Setup error! :( \n  {}", e),
    }

    match gather_data(){
        Ok(result) => println!("Gathering data succesfull"),
        Err(e) => eprintln!("Data gathering error! :( \n  {}", e),
    }
}


