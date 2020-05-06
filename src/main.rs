extern crate orbtk;
use orbtk::prelude::*;

use fs::read_to_string;
use std::fs;

use debukit::*;

fn main() {
    Application::new()
    .window(|ctx| {
        Window::create()
            .title("OrbTk - minimal example")
            .position((100.0, 100.0))
            .size(1024.0, 768.0)
            .child(TextBlock::create().text("debukit").build(ctx))
            .build(ctx)
    })
    .run();
}
