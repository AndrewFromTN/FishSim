mod dioxus_app;
mod topography;

use topography::TopographicMap;

fn main() {
    //dioxus::launch(App);

    const SEED: u32 = 42;
    const WIDTH: usize = 96;
    const HEIGHT: usize = 64;
    const SCALE: f64 = 0.12;

    let map = TopographicMap::new(SEED, WIDTH, HEIGHT, SCALE);
    println!("{}", map);
}
