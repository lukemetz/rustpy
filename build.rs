use std::io::Command;

fn main() {
  Command::new("make").arg("-C").arg("src").status().unwrap();
}
