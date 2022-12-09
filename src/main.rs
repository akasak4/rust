use std::io;
fn main() {
println!("Siapa Namamu");
println!("Tolong masukkan namamu.");
let mut guess = String::new();
io::stdin().read_line(&mut guess)
.expect("Failed to read line");
println!("Salam Kenal {}", guess);
}
