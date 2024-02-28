// Define a function to concatenate two string slices and return a new String
fn concatenate_strings(string1: &str, string2: &str) -> String {
    // Create an empty String to store the concatenated result
    let mut result = String::new();

    // Append the contents of the first string slice to the result String
    result.push_str(string1);
    
    // Append the contents of the second string slice to the result String
    result.push_str(string2);

    // Return the concatenated String
    result
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    // Call the concatenate_strings function with references to string1 and string2
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the concatenated string
    println!("{}", concatenated_string);
}
