fn variable_scope(){
    /*
    {                      // "s" is not valid here, it’s not yet declared
        let s = "hello";   // "s" is valid from this point forward

        // sで作業をする
    }                      // this scope is now over, and s is no longer valid
    */


    let mut s = String::from("hello");

    s.push_str(", world"); // push_str() function appends a literal to a String

    println!("{}", s); // This will print "hello, world"

}

fn string_type(){
    let x = 5;
    let y = x;

    println!("x:{}, y:{}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2:{}", s2);
    //println!("s1:{}", s1); // "s1" is already moved to s2. so this line is error

}

fn memory_and_allocation(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}",s1, s2);
}



fn ownership_and_function(){
    let hello = String::from("hello");  // hello comes into scope

    takes_ownership(hello); // hello's value move into the function...
                            // ... and so is no longer valid here.

    let x = 5;              // x comes into scope

    makes_copy(x);          // x would move into the function,
                            // but i32 is Copy, so it's okay to still use x afterward.
}

fn takes_ownership(some_string: String){ // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope, and "drop" is called. the backing memory is freed.

fn makes_copy(some_integer: i32){ // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



fn return_values_and_scope_1(){

    let s1 = gives_ownership();         // gives_ownership moves its retrun value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also
                                        // moves its return value into s3.
    println!("s1:{}, s3:{}", s1,s3);    // println!("s2:{}", s2); is punished.
    
}   // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved,
    // so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String{ // gives_ownership will move its return value into the 
                                // function that calls it
    
    let some_string = String::from("hello");

    some_string
} 

fn takes_and_gives_back(a_string: String) -> String{ // a_string comes into scope.

    a_string // a_string is returned and moves out to the calling function
} 

fn return_values_and_scope_2(){
    let s1 = String::from("hello");

    let (s2, len) = caluculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn caluculate_length(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}

fn main() {
    variable_scope();
    string_type();
    memory_and_allocation();
    ownership_and_function();
    return_values_and_scope_1();
    return_values_and_scope_2();
}
