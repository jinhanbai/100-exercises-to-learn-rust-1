// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`. (struct Order)
//   The product name can't be empty and it can't be longer than 300 bytes. (val)
//   The quantity must be strictly greater than zero. (val)
//   The unit price is in cents and must be strictly greater than zero. (val)
//   Order must include a method named `total` that returns the total price of the order. (func)
//   Order must provide setters and getters for each field. (funcs)
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.


pub mod order {
    //use std::iter::Product;
    //use std::ascii::AsciiExt;


//use std::str;
    pub struct Order {
        product_name: String,
        quantity: u64, //already enforces non-negativity
        unit_price: u64,
    }

    impl Order {
        pub fn new(product_name: String, quantity: u64, unit_price: u64) -> Order {
            if product_name.is_empty() {
                panic!("Product name can not be empty!");
            }
            if product_name.len() > 300 {
                panic!("Product name can not exceed 300 bytes.");
            }
            if quantity == 0 {
                panic!("Quantity can not be negative!")
            }
            if unit_price == 0 {
                panic!("Unit price can not be negative!")
            }
            Order {product_name,
                 quantity, 
                 unit_price}
        }
    }
}
