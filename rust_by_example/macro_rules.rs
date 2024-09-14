macro_rules! say_hola {
    // () signifies no arguments
    () => {
        // the macro will expand into the contents of this block
        println!("Hola Hola");
    };
}
fn main() {
    // this call will expand to println!("Hola Hola");
    say_hola!();
}
