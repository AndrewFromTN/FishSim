use dioxus::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Clone, Debug)]
struct Fish {
    id: usize,
    age: u32,
    alive: bool,
}

impl Fish {
    fn new(id: usize) -> Self {
        Fish {
            id,
            age: 0,
            alive: true,
        }
    }

    fn step(&mut self, rng: &mut StdRng, death_rate: f64) {
        self.age += 1;
        if rng.random_bool(death_rate) || self.age > 10 {
            self.alive = false;
        }
    }
}

#[derive(Debug, Clone)]
struct FishSimulation {
    fish: Vec<Fish>,
    next_id: usize,
    rng: StdRng,
    death_rate: f64,
    spawn_threshold: usize,
    spawn_count: usize,
    history: Vec<usize>,
}

impl FishSimulation {
    fn new_with_seed(
        initial_count: usize,
        death_rate: f64,
        spawn_threshold: usize,
        spawn_count: usize,
        seed: u64,
    ) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        let fish = (0..initial_count).map(Fish::new).collect();
        FishSimulation {
            fish,
            next_id: initial_count,
            rng,
            death_rate,
            spawn_threshold,
            spawn_count,
            history: vec![initial_count],
        }
    }

    fn step(&mut self) {
        for fish in &mut self.fish {
            if fish.alive {
                fish.step(&mut self.rng, self.death_rate);
            }
        }

        let alive_count = self.fish.iter().filter(|f| f.alive).count();
        if alive_count < self.spawn_threshold {
            self.spawn_fish(self.spawn_count);
        }
        self.history.push(self.population_count());
    }

    fn spawn_fish(&mut self, count: usize) {
        for _ in 0..count {
            self.fish.push(Fish::new(self.next_id));
            self.next_id += 1;
        }
    }

    fn alive_fish(&self) -> Vec<&Fish> {
        self.fish.iter().filter(|f| f.alive).collect()
    }

    fn population_count(&self) -> usize {
        self.alive_fish().len()
    }

    fn history(&self) -> &[usize] {
        &self.history
    }
}

#[component]
fn App() -> Element {
    let mut seed = use_signal(|| 42u64);
    let mut sim = use_signal(|| FishSimulation::new_with_seed(20, 0.1, 10, 5, *seed.read()));
    let mut tick = use_signal(|| 0u64);
    let mut autoplay = use_signal(|| false);
    let mut death_rate = use_signal(|| 0.1f64);
    let mut spawn_threshold = use_signal(|| 10usize);
    let mut spawn_count = use_signal(|| 5usize);

    use_future(move || async move {
        loop {
            if *autoplay.read() {
                let simulation = &mut sim.write();
                simulation.death_rate = *death_rate.read();
                simulation.spawn_threshold = *spawn_threshold.read();
                simulation.spawn_count = *spawn_count.read();
                simulation.step();

                tick += 1;
            }

            gloo_timers::future::TimeoutFuture::new(500).await;
        }
    });

    let chart_data = serde_json::to_string(sim.read().history()).unwrap();

    rsx! {
        div { class: "p-4 space-y-4",
            h1 { class: "text-2xl font-bold", "Fish Population Simulation" }

            div { class: "flex gap-4 flex-wrap",
                button {
                    class: "bg-blue-500 text-white px-4 py-2 rounded",
                    onclick: move |_| {
                        let simulation = &mut sim.write();
                        simulation.death_rate = *death_rate.read();
                        simulation.spawn_threshold = *spawn_threshold.read();
                        simulation.spawn_count = *spawn_count.read();
                        simulation.step();

                        tick += 1;
                    },
                    "Step"
                }
                button {
                    class: "bg-green-500 text-white px-4 py-2 rounded",
                    onclick: move |_| autoplay.toggle(),
                    if *autoplay.read() { "Pause" } else { "Autoplay" }
                }
                button {
                    class: "bg-red-500 text-white px-4 py-2 rounded",
                    onclick: move |_| {
                        sim.set(FishSimulation::new_with_seed(20, *death_rate.read(), *spawn_threshold.read(), *spawn_count.read(), *seed.read()));
                        tick.set(0);
                    },
                    "Reset Simulation"
                }
            }

            div { class: "grid gap-2",
                label { "Seed: {seed.read()}" }
                input {
                    r#type: "number",
                    value: "{seed.read()}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<u64>() {
                            seed.set(val);
                        }
                    }
                }
                button {
                    class: "bg-yellow-500 text-white px-4 py-2 rounded",
                    onclick: move |_| {
                        sim.set(FishSimulation::new_with_seed(20, *death_rate.read(), *spawn_threshold.read(), *spawn_count.read(), *seed.read()));
                        tick.set(0);
                    },
                    "Apply New Seed"
                }

                label { "Death Rate: {death_rate.read():.2}" }
                input {
                    r#type: "range",
                    min: "0.01", max: "0.9", step: "0.01",
                    value: death_rate.read().to_string().as_str(),
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<f64>() {
                            death_rate.set(val);
                        }
                    }
                }
                label { "Spawn Threshold: {spawn_threshold.read()}" }
                input {
                    r#type: "range",
                    min: "1", max: "50", step: "1",
                    value: spawn_threshold.read().to_string().as_str(),
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<usize>() {
                            spawn_threshold.set(val);
                        }
                    }
                }
                label { "Spawn Count: {spawn_count.read()}" }
                input {
                    r#type: "range",
                    min: "1", max: "20", step: "1",
                    value: "{spawn_count.read()}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<usize>() {
                            spawn_count.set(val);
                        }
                    }
                }
            }
            p { "Tick: {tick}" }
            p { "Population: {sim.read().population_count()}" }
            iframe {
                class: "w-full h-64 border",
                srcdoc: (format!("<html><body><pre>{}</pre></body></html>", chart_data)).as_str()
            }
            ul {
                for fish in sim.read().alive_fish().iter() {
                    li { "Fish #{fish.id} - Age: {fish.age}" }
                }
            }
        }
    }
}
