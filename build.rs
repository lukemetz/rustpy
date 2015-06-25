extern crate gcc;

fn main() {
    gcc::compile_library("libmacroexpand.a", &["src/macroexpand.c"]);
}
