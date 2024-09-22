// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    /* get_char()函数接收一个String类型参数，并消耗掉它的所有权，因为后面函数需要使用它的引用，
	   因此要传入一个克隆副本 */
    get_char(data.clone());

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    let data: &str = &data.to_uppercase();

    println!("{}", data);
}
