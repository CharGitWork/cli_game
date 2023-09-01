use console::Term;

fn main() {
    let console = Term::buffered_stdout();
    for _ in 0..10 {
        console.write_line("Salutations").unwrap();
        console.write_line("Rustacés").unwrap();
    }
    println!("ok");
    console.flush().unwrap();
}