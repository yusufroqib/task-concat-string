fn main() {
    println!("Hello, world!");
    let string1 = String::from("I am");
    let string2 = String::from(" a boy");

    let concatenated_string = concatenate_strings( &string1, &string2); 

    println!("{}", concatenated_string);
}

fn concatenate_strings(s1: &String, s2:&String ) -> String {
    let mut new_string = s1.clone();
    new_string.push_str(s2);
    new_string
}