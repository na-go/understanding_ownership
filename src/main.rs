


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

    let (s2, len) = caluculate_length_1(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn caluculate_length_1(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}

fn references_and_borrowing(){
    let s1 = String::from("hello");

    let len = caluculate_length_2(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn caluculate_length_2(s: &String) -> usize { // s is a reference to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of 
    // what it refers to, nothing happens.

fn mutable_references(){
    let mut s = String::from("hello");

    change_s(&mut s);

    println!("{}", s);
}

fn change_s(some_string: &mut String){ // &mut String change &String, which has error.
    some_string.push_str(", world!");
}

fn dangling_references(){
    /*
    ## Bad Example ##
    ## this function's return type contains a borrowed value, but there is no   ##
    ## value for it to be borrowed from                                         ##
    fn main(){
        let reference_to_nothing = dangle();
    }

    fn dangle() -> &String {            // dangle retruns a reference to String
        let s = String::from("hello");  // s is new String

        &s                              // Return a reference to String s 
    }   // Here, s goes out of scope, and is dropped. Its memory goes away. Danger.
    */

    let references_to_nothing = no_dangle();

    println!("{}", references_to_nothing);
}

fn no_dangle() -> String{
    let s = String::from("hello");
    
    s
}

fn the_slice_type(){
    let mut s = String::from("hello world!");

    let word = first_word(&s);  // word will get the value 5.

    println!("{}", word); 
    s.clear();                  // this empties the String, making it equal to "".
    /*
    ## "word" still has the value 5 here, but there's no more string that           ##
    ## we could meaningfuly use the value 5 with. "word" is now totally invalid!    ##
    */
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slices_1(){
    let s = String::from("hello world");

    let firstword = first_word_2(&s);
    println!("{}", firstword);
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slices_2(){
    let s = String::from("hello world suprise");

    let secondword = second_word(&s);
    println!("secondword:{}", secondword);
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_number = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && first_number == 0 {
            first_number = i;
        }
        
        if item == b' ' && first_number != 0 && first_number != i{
            return &s[first_number..i]
        }
        
    }
    &s[first_number..]
}

fn main() {
    // 4.1
    variable_scope();
    string_type();
    memory_and_allocation();
    ownership_and_function();
    return_values_and_scope_1();
    return_values_and_scope_2();
    // 4.2
    references_and_borrowing();
    mutable_references();
    dangling_references();
    // 4.3
    the_slice_type();
    string_slices_1();
    string_slices_2();
}
