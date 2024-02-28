# RUST-Tasks
We define a function concatenate_strings that takes two string slices (&str) as arguments and returns a new String.
Inside the function, we create an empty String called result to store the concatenated string.
We use the push_str method to append the contents of the first string slice (string1) and then the second string slice (string2) to the result string.
The function returns the result string.
In the main function, we create two String variables, string1 and string2, with the desired initial values.
We then call the concatenate_strings function with references to string1 and string2.
The result is stored in the concatenated_string variable, which we print to the console using println!.
This program demonstrates ownership, borrowing, and references by passing references to the original strings to the concatenate_strings function, allowing us to concatenate them without transferring ownership.
