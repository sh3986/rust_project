use std::str::Split;

fn main() {
    let text: &str = "Manchester Bogota Paris Dallas Chicago";
    let cities: Split<&str> = text.split(" ");

    for city in cities {
        println!("City {}", city);        
    }
}
