#[macro_use]

pub mod vm;
pub mod assembler;
pub mod instruction;
pub mod repl;

fn main() {
    let mut repl = repl::REPL::default();
    repl.run();
}
