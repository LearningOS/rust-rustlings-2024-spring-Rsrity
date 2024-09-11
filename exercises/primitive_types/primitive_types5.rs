// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.



fn main() {
	/* 定义了一个元组，包含一个字符串和一个浮点数 */
    let cat = ("Furry McFurson", 3.5);
	/* 解构元组，将其分解为两个变量，分别为name和age */
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
