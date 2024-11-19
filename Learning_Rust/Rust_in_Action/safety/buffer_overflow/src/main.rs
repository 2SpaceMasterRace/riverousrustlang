fn main() {
    let fruits = vec!["kiwi", "bananna", "grapes"];
    let buffer_overflow = fruits[4];
    assert_eq!(buffer_overflow, "watermelon");
}

