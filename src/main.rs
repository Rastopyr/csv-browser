extern crate qmlrs;

fn main() {
  println!("Hello, world!");

  let mut engine = qmlrs::Engine::new();

  engine.load_local_file("src/gui/window.qml");

  engine.exec();
}
