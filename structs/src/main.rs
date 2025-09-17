fn main() {
    basics();
    coping();
    tuples();
    unit_type_struct();
    rectangles();
    method_fun();
}

fn  method_fun(){
    #[derive(Debug)]
    struct Rec {
        w: u32,
        h: u32
    }

    impl Rec {
        fn area(&self) -> u32 {
            &self.w * &self.h
        }

        fn can_hold(&self, o: &Rec) -> bool {
            self.area() >= o.area()
        }

        fn square(i: u32) -> Self {
            Self {
                w: i,
                h: i
            }
        }
    }

    // można mieć wiele bloków impl
    impl Rec {
        // new nie jest słowem kluczowym, więc od biedy można zrobić taki bieda konstruktor a'la
        // record. Ma to swój urok.
        fn new(w: u32, h: u32) -> Self {
            Self{
                w: w,
                h: h
            }
        }
    }

    let rec = Rec::new(2, 4);
    let sq = Rec::square(2);

    println!("Area of {rec:?} is {0}", rec.area());
    println!("Can {rec:?} hold {sq:?}? {0}", rec.can_hold(&sq));
}

fn rectangles(){
    #[derive(Debug)]  // to idzie
    struct Rectangle{
        w: u32,
        h: u32
    };
    fn area(r: &Rectangle) -> u32 {
        r.w * r.h
    }
    let rec = Rectangle{
        w: 3,
        h: 4
    };

    println!("Area is {0}", area(&rec));
    println!("Rec is {rec:?}"); // z tym. Taki domyślny toString
    println!("Rec is {rec:#?}");
    dbg!(&rec); // to idzie na stderr
    
}
fn  unit_type_struct(){
    struct AlwaysEq;
    let _aeq = AlwaysEq;

    // to nie zadziała, po aeq nie ma standardowego formatera
    //  println!("{0}", _aeq);
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
    let pan_sowa = build_user_short(String::from("Pan Sowa"), String::from("radosc_z_chlopcami@domek.las"));

    print_user(jas);
    
    malgosia = print_user( malgosia);
    malgosia.username = String::from("Dla ciebie Małgorzata");
    print_user(malgosia);
 
    println!("{0} {1}", baba_jaga.email, baba_jaga.username);
    println!("{0} {1}", pan_sowa.email, pan_sowa.username);
}

fn build_user(username: String, email: String) -> User {
    User {
       active: false,
       username: username,
       email: email,
       sign_in_count: 0
    }
}

fn build_user_short(username: String, email: String) -> User{
    User {
        active: false,
        username,
        email,
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
