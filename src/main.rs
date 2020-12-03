mod selection;

fn main() {
    let v = vec![3, 2, 1, 10, 9, 8, 7];
    let result = selection::selection_sort(&v);
    println!("{:?}", result);
}
