fn main() {
    // Declared in the outer scope, so it's visible in the inner block too.
    let greeting = "Hello";

    {
        println!("{greeting} from inside the block");
    }

    println!("{greeting} from outside the block");
}
