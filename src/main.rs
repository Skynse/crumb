mod engine;
mod interface;

use engine::Engine;
use interface::Interface;
fn main() {
    let engine = Engine::new();
    Interface::run(engine);
}
