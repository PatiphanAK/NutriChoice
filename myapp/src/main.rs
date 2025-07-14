mod entities;

use entities::product::Product;

fn main() {
    let p = Product {
        id: 1,
        name: String::from("Apple"),
        price: 3.5,
    };
    println!("{:?}", p);
}
