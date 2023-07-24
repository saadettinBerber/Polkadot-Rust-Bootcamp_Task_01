fn main() {
    let string1: &str = "Rust";                                                
    let string2: &str = "Ceans";
    let concatenated_string : String = String::from(concatenate_strings(string1, string2));          
    println!("Result : {}", concatenated_string);
}

fn concatenate_strings(string1:&str, string2:&str)-> String{
    let mut result: String = String::from(string1.clone()); 
    result.push_str(string2);
    result
}
