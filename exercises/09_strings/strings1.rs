// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    let mut s: String = String::new();
    let data = "blue";
    s = data.to_string();
    return s;
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
