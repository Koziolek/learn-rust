fn main() {
    // let x = 5;
    let mut  x = 5;
    println!("Hello, world!");
    x = 6;

    println!("The value of {x}");
    const THREE_HOURS_IN_SEC: u32 = 60* 60 * 3;

    println!("3h in sec: {THREE_HOURS_IN_SEC}");

    shadowing();
    type_shadowing();
    tuples();

    let arr: [i32; 5] = [0, 1, 2, 3, 4];
    let over: usize = overflow_reason(&arr, false);
    arrays(&arr, over);
    infinite_loop_with_labels();
    while_loop();
    for_loop(&arr);
    ranges();
    ownership_magic();
}



fn shadowing(){
    let x = 5;
    let x = x + 1;
     println!("External scope {x}");
    {
        let x = 2*x -1;
        println!("Internal scope {x}");
    }
     println!("External scope {x}");
}

fn type_shadowing() {
    // let mut spaces = "   "; // will not work shadowing of mut forbid type change
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces is {spaces}");

}

fn tuples() {
    let tuple: (u32, f64, bool) = (500, 5.6, true);


    // println!("Tuple is  {tuple}");
    
    println!("Tuple is {2}, {1}, {0}",  tuple.0, tuple.1, tuple.2);
    let (x, y, z) = tuple;
    println!("Tuple is  {x} {y} {z}");
}

fn arrays(arr2: &[i32], i: usize){
    let arr: [i32; 5] = [0, 1, 2, 3, 4];

    // println!("{0} {1}", arr[0], arr2[5]); // direct compilation error

    println!("{0} {1}", arr[0], arr2[0]);
    if i < arr2.len() {
        println!("{0} {1}", arr[0], arr2[i]);   // runtime error w dev mode. w prod mode będzie dupa
    } else {
        println!("{0} {1}", arr[0], arr2[i]);   // runtime error w dev mode. w prod mode będzie dupa
    }

}

fn overflow_reason(a: &[i32], breaking: bool) -> usize {
    if breaking {
        a.len() + 1 // jebaniutki… z ; jest statement, ale bez jest expression
    } else {
        a.len() - 1
    }
}

fn infinite_loop_with_labels(){
    let mut count  = 0;

    'count_up: loop {
        println!("Counter: {count}");
        
        let mut remaining = 10;

        loop {
        println!("Remaining: {remaining}");
            if remaining == 9 {
                break;
            } 
            if count == 2{
                break 'count_up;
            }
            remaining -= 1;
        }

        count += 1;

    }

    println!("Counter at the end {count}");
}

fn while_loop() {
    let mut count = 0;
    while count < 3 {
        println!("While count {count}");
        count+=1;
    }
    

    println!("While count at the end: {count}");
}

fn for_loop(arr: &[i32]){
    for el in arr{
        println!("For el: {el}");
    }

}

fn ranges() {
    for n in (1..4).rev() {
        println!("Range element {n}");
    }
}

fn ownership_magic(){
    let s1 = String::from("hello");
    consume_string(s1);
    // println!("s1: {s1}"); // poza zakresem
    
    let s2 = String::from("Ahoj");
    consume_and_poop(s2);
    // println!("s2: {s2}"); // też poza zakresem
    let s3 = String::from("Hej");
    consume_and_poop(s3.clone());
    println!("s3: {s3}"); 

    let s4 = String::from("konichiwa");
    consume_ref(&s4);
    println!("s4: {s4}");

    let mut s5 = String::from("Bonjour");
    consume_ref_and_modify(&mut s5);
    println!("s5: {s5}");
}

fn consume_string(s: String ) {
    println!("Consumed s: {s}");
}
fn consume_and_poop(s: String) -> String {
    println!("Consumed and pooped s: {s}");
    s
}
fn consume_ref(s: &String) {
    println!("Consume ref: {s}");
}
fn consume_ref_and_modify(s: &mut String){
    s.push_str(", chuju");
}

