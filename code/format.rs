fn main() {
    // println is used to print to std in with newline
    // by default {} will be replaced by the argument provided
    // argument is stringified
    println!("{} days", 31);

    // if there is no suffix provided 31 becomes i32(int 32)
    // it can be changed by provided suffix like i64(31i64)

    // {} can also be used as postional argument
    println!("{0} is older than {1}, {1} is younger than {0}", "Vincent", "singh");

    // works same for named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             verb="jumps over",
             subject="the quick brown fox");
    
    // special formatting can be specified after a `:`
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    // in above line :b represents the binary of 2 - (10)

    // you can right-align text with a specified width. This will output
    // "     1". 4 white spaces and a "1"
    println!("{number:>width$}", number=1, width=5);

    // you can pad with zeros(really important for sorting algorithms)
    // this will add 4 zeros in front of 6 (00006)
    println!("{number:>0width$}", number=6, width=5);

    // rust also checks to make sure the correct number of arguments are given
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // create a structure named `Structure` which contain an i32
    #[allow(dead_code)]
}
