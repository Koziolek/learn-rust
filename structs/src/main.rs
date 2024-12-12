fn main() {
    basics();
    coping();
    tuples();

}


fn tuples(){
    struct Color(i32, i32, i32);   
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let start = Point(0, 0, 0);

    // print_t3(black); // ta sztuczka nie działa w ten sposób potrzebny trait From<>
    // https://stackoverflow.com/questions/53194323/is-there-any-way-of-converting-a-struct-to-a-tuple

//    let mix = Mix (1, 2, String::from("MIX"));
//    print_t3(mix);
}
// -- offtopic z traitami
struct Mix(i32, i32, String);

//impl From<Mix> for (i32, i32, String) {
//   fn from(m: Mix)-> (i32, i32, String){
//        let mix{a, b, c}: Mix = m;
//        (a, b, c)
//    }  
//}

fn print_t3(t: (i32, i32, String)) -> (i32, i32, String){
    println!("({0}, {1}, {2})", t.0, t.1, t.2);
    t
}
// -- koniec

fn coping() {
    let jan = build_user(String::from("Jan"), String::from("jan@jan.jan"));
    let jan = print_user(jan);
    let jasio = User{
        username: String::from("Jasio"),
        ..jan
    };
    print_user(jasio);
}

fn basics() {

    let jas = User{
        active: false,
        username: String::from("jaś"),
        email: String::from("jaś@małogosia.las"),
        sign_in_count: 0
    };

    let mut malgosia = User {
        active: false,
        username: String::from("małgosia"),
        email: String::from("małgosia@jaś.las"),
        sign_in_count: 0
    };

    let baba_jaga = build_user(String::from("Chuj"), String::from("abrakadabrakonstantynopolitowianeczkatrzy@wskrócie.zwana"));

    print_user(jas);
    
    malgosia = print_user( malgosia);
    malgosia.username = String::from("Dla ciebie Małgorzata");
    print_user(malgosia);
 
    println!("{0} {1}", baba_jaga.email, baba_jaga.username);
}

fn build_user(username: String, email: String) -> User {
    User {
       active: false,
       username: username,
       email: email,
       sign_in_count: 0
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn print_user(user: User) -> User {
    println!("User: {{ 
        username: {0}
        email: {1}
        active: {2}
        sign_in_count: {3}
        }}
        ", user.username, user.email, user.active, user.sign_in_count);

    user
}
