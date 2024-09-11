mod helper;
use helper::stream::Stream;

mod dom;
use dom::parser::tokenizer;

fn main() { 
    // Test data: a simple slice of integers
    let data = [1, 2, 3, 4, 5];
    let mut stream = Stream::new(&data);

    // Test advancing and accessing the current element
    println!("Initial stream: {:?}", stream);
    println!("Current element: {:?}", stream.current()); // Should print Some(1)

    stream.advance();
    println!("After advancing: {:?}", stream.current()); // Should print Some(2)

    // Test expect_and_skip
    if stream.expect_and_skip(2).is_some() {
        println!("Matched 2 and advanced!"); // Should print this
    } else {
        println!("Did not match 2.");
    }
    println!("Current element: {:?}", stream.current());
    // Test expect_oneof_and_skip
    if stream.expect_oneof_and_skip(&[3, 4]).is_some() {
        println!("Matched 3 or 4 and advanced!"); // Should print this with 3
    } else {
        println!("Did not match 3 or 4.");
    }
    println!("Current element: {:?}", stream.current());
    // Test slicing methods
    println!("Slice from 1 to 3: {:?}", stream.slice(1, 3)); // Should print [2, 3]
    println!("Checked slice from 1 to 10: {:?}", stream.slice_checked(1, 10)); // Should print [2, 3, 4, 5]

    // Test end of stream
    while !stream.is_eof() {
        println!("Current: {:?}", stream.current());
        stream.advance();
    }
    println!("Reached end of stream");
}
