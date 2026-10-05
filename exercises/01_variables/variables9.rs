fn main() {
    {
        let greeting = "Hello";
        println!("{greeting} from inside the block");
    }

    // TODO: `greeting` doesn't exist here. Move its declaration so that
    // both `println!` calls work.
    println!("{greeting} from outside the block");
}
