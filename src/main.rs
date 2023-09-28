// mod include the whole module
mod lib;

fn main() {
    println!("Hello, wolrd!");
    lib::sayHi();
    // dot product
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    assert_eq!(lib::dot_product(&a1, &a2), 1 * 4 + 2 * 5 + 3 * 6)
}
