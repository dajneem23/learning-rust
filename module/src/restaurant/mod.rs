mod pizza_order {
    pub struct Pizza {
        pub size: String,
        pub toppings: Vec<String>,
    }
    impl Pizza {
        pub fn new(size: String) -> Self {
            Pizza {
                size,
                toppings: Vec::new(),
            }
        }
        pub fn add_topping(&mut self, topping: String) {
            self.toppings.push(topping);
        }
    }
    pub mod help_customer {
        pub fn how_to_order() {
            println!("Choose the size of the pizza.");
            println!("Add toppings to the pizza.");
            println!("Place the order.");
        }
        pub fn take_payment() {
            println!("Take payment for the order.");
        }
        pub fn take_order() {
            println!("Take the customer's order.");
            let cust_pizza = super::Pizza::new(String::from("Large"));
            println!("Pizza size: {}", cust_pizza.size);
            serve_order();
        }
        pub fn serve_order() {
            println!("Serve the customer's order.");
        }
    }
}
pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
