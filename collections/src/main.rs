fn main() {

    // Tworzenie za pomocą konstruktora 
    let mut v: Vec<i32> = Vec::new();

    println!("{v:?}");
    // i makra
    v = vec![1, 2, 3];
    println!("{}", &v[1]);

    // dodawanie do wektora
    let v1 = pushing();

    println!("{}", &v1[1]);

    // odczyt z wektora
    reading();

    // magia pożyczania i widocznośc
    borrowing();

    // iterowanie się po wektorze
    iter();    
    
    // wykorzystanie enuma do składowania różnych wartości
    enums::call();
    
    // Mapy
    maps::call();
}

// --- Mapy

pub mod maps{
    use std::collections::HashMap;

    pub fn call(){
        // tworzenie mapy
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("green"), 12);

        // dostęp po kluczu
        
        let team_name = String::from("blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

        println!("{team_name} team scored {score}");

        // itearacja
        
        for (key, value) in &scores {
            
            println!("{key} team scored {value}");
        } 

        // to nie zadziała wg. podręcznika, ale jednak działa!
        let key_val = String::from("key");
        let val_val = String::from("val");
        let mut map = HashMap::new();
        map.insert(key_val, val_val);
        
        println!("{map:?}");
        //
        //
    }
}

// --- wykorzystanie enuma do składowania wartości
pub mod enums {
    
    pub fn call(){
        let row = vec![
            SpreadsheetCell::Number(1),
            SpreadsheetCell::Text(String::from("some text")),
            SpreadsheetCell::Boolean(true),
            SpreadsheetCell::Float(1.1),
        ];
        for i in row {
            println!("{i:?}");
        }
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Number(i32),
        Text(String),
        Boolean(bool),
        Float(f64),
    }
}

// --- iterowanie się po wektorze
fn iter(){
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    // aktualizacja wartości 
    mut_iter()
}

fn mut_iter() {
    let mut v = vec![100, 32, 57]; // wektor musi być mutowalny
    for i in &mut v { // aktualna zmienna jest mutowalna
        *i += 50; // dereferencja do wartości
    }

    for i in &v {
        println!("{i}");
    }


}

// --- magia pożyczania i widoczności
fn borrowing() {
    let mut v = vec![1, 2, 3];
    let first = &v[0]; // odczyt bez mut 
    
    println!("Pierwszy to {first}"); // to zadziała
    
    v.push(4); // dodajemy coś do wektora

    // println!("Pierwszy to {first}"); // i kompilacyjna dupa
    // Problem polega na tym, że vektor jest składowany jak tablica i w przypadku gdy dodajemy
    // element to może być potrzeba przeniesienia całego obszaru pamięci w inne miejsce. W takim
    // przypadku first wskazywał będzie na zdealokowany fragment pamięci. 
}
// --- odczyt wektora
fn reading(){
    let v = vec![1, 2, 3, 4];
    let third: &i32 = &v[2];
    println!("trzeci element to {third}");
    
    let third2: Option<&i32> = v.get(2);
    match third2 {
        Some(val) => println!("trzeci element to {val}"),
        None => println!("Trzeciego elementu brak!"),
    }

    // let _hun: &i32 = &v[100]; // to tu nie zadziała i będzie panika
    // println!("{_hun}");

    let o_hun: Option<&i32> = v.get(100);
    // to zadziała jak każda monada
    match o_hun {
        Some(val) => println!("Setny element to {val}"),
        None => println!("Setnego elementu brak!"),
    }

}

// --- dodawanie do wektora
fn pushing() -> Vec<i32> {
    // musi być mut, bo zmieniamy stan
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    v
}
