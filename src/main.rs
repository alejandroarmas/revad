
pub mod chain;
pub mod tape;

use tape::Tape;


fn main() {

    let t = Tape::new();

    let x = t.var(0.5);
    let y = t.var(4.2);

    let z = x * y + x.sin();

    let grad = z.grad();

    println!("z = {}", z.value());         // z = 2.579425538604203
    println!("∂z/∂x = {}", grad.wrt(x)); // ∂z/∂x = 5.077582561890373
    println!("∂z/∂y = {}", grad.wrt(y)); // ∂z/∂y = 0.5
}