use day25::CodeGen;

fn main() {
    let row = 2947;
    let col = 3029;
    let mut cg = CodeGen::default();
    println!("Code: {}", cg.get(row, col));
}
