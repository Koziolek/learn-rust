mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist(){}
        pub fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }

}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // ścieżka absolutna
    crate::front_of_house::hosting::add_to_waitlist();

    // ścieżka relatywna
    
    front_of_house::hosting::seat_at_table();
    
    // widoczność struktur
    let mut meal = back_of_house::Breakfast::summer("Wheat");
    println!("I would like to eat with {}", meal.toast);
    // nadpisujemy publiczną część mut-struktury. Mutowalność jest tutaj tranzytywna 
    meal.toast = String::from("Rye");
    println!("I would like to eat with {}", meal.toast);

    let order1 = back_of_house::Apetizer::Soup;
    let order2 = back_of_house::Apetizer::Salad;

    // od tego miejsca piszemy z wykorzytaniem use
    hosting::add_to_waitlist();

}

mod customer {
    // odpowiednik statycznego importu - tak nie robimy
    use crate::front_of_house::hosting::add_to_waitlist;

     // i tutaj już nie działa nasz use z linii 15
    pub fn eat_at_restaurant() {
        // to się wywali
        // hosting::add_to_waitlist();
        // albo przenosimy use do tego modulu, albo:
        super::hosting::add_to_waitlist();
        // wywołanie zaimportowaniej funkcji – nie wiadomo skąd ona się tu wzięła!
        add_to_waitlist();

    }
    
}

fn deliver_order(){}

mod back_of_house{
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order(){}
    
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // To jest metoda fabrykująca, ponieważ struct zawiera prywatne pola, których nie ustawimy
        // poza modułem!
        pub fn summer(toast: &str) -> Breakfast {
           Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // W enumie wszystkie wartości są publiczne
    pub enum Apetizer {
        Soup,
        Salad,
    }
}

// shadowing importów

use std::fmt;
use std::io;

//fn function1() -> fmt::Result<> {}

//fn function2() -> io::Result<()>{}

// importy nazwane

use std::fmt::Result;
use std::io::Result as IoResult;

// reeksport - pozwala na zmianę „mentalu” kodu 

pub use crate::front_of_house::hosting as waiter;

// module customer możemy użyć restaurant::hosting::add_to_waitlist zmieniając architekturę
// rozwiązania, bo customer myśli kategoriami restauracja»kelner, a nie restauracja»kuchnia»kelner. 
