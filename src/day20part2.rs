use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pulse {
    LOW,
    HIGH,
}

impl Pulse {
    fn inv(&self) -> Pulse {
        match self {
            Pulse::LOW => Pulse::HIGH,
            Pulse::HIGH => Pulse::LOW,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Signal {
    source: String,
    destination: String,
    pulse: Pulse,
}

#[derive(Clone)]
struct Machine {
    modules: std::collections::HashMap<String, Box<dyn Module>>,
    state_cache: std::collections::HashMap<Machine, (Machine, Vec<Signal>)>,
}

impl PartialEq for Machine {
    fn eq(&self, other: &Self) -> bool {
        let mut s1 = DefaultHasher::new();
        let mut other_s = DefaultHasher::new();

        self.hash(&mut s1);
        other.hash(&mut other_s);

        return s1.finish() == other_s.finish();
    }
}

impl Eq for Machine {}

impl Hash for Machine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut module_ids = self.modules.keys().collect::<Vec<_>>();
        module_ids.sort_unstable();

        let mut s = DefaultHasher::new();
        for module_id in module_ids {
            self.modules.get(module_id).unwrap().my_hash(&mut s);
        }
        let sub_hash = s.finish();
        sub_hash.hash(state);
    }
}

impl Machine {
    fn from_str(input: &str) -> Machine {
        let module_tuples: Vec<(String, Box<dyn Module>)> = input
            .trim()
            .lines()
            .map(|line| {
                let line = line.trim();
                let line_parts: Vec<_> = line.split(" -> ").collect();

                let type_and_id = line_parts[0];
                let destination_list = line_parts[1];

                let module_type = &type_and_id[0..1];
                let module_id = if module_type == "b" {
                    &type_and_id[..]
                } else {
                    &type_and_id[1..]
                };

                let destinations: Vec<String> = destination_list
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();

                let base = ModuleBase::new(&module_id.to_string(), destinations);

                let module = match module_type {
                    "b" => Box::new(Broadcast::new(base)) as Box<dyn Module>,
                    "%" => Box::new(FlipFlop::new(base)) as Box<dyn Module>,
                    "&" => Box::new(Conjunction::new(base)) as Box<dyn Module>,
                    _ => panic!("Unknown module_type_id"),
                };

                return (module_id.to_string(), module);
            })
            .collect();

        let mut machine = Machine {
            modules: std::collections::HashMap::from_iter(module_tuples.into_iter()),
            state_cache: std::collections::HashMap::new(),
        };

        machine.init_inputs();

        return machine;
    }

    fn init_inputs(&mut self) {
        // collect src -> dst mappings
        let mut mappings: Vec<(String, Vec<String>)> = Vec::new();
        for (src_module_id, src_module) in &self.modules {
            mappings.push((src_module_id.clone(), src_module.get_destinations().clone()));
        }

        for (src_module_id, dst_module_ids) in mappings {
            for dst_module_id in dst_module_ids {
                match self.modules.get_mut(&dst_module_id) {
                    Some(module) => module.add_input(&src_module_id),
                    None => {}
                };
            }
        }
    }

    fn tick(&mut self) -> (usize, usize) {
        let mut next_pending_pulses = Vec::new();

        for pending_pulse in &self.pending_pulses {
            match self.modules.get_mut(&pending_pulse.destination) {
                Some(module) => next_pending_pulses
                    .extend(module.receive(pending_pulse.pulse, &pending_pulse.source)),
                None => {}
            };
        }

        let mut low_pulse_count = 0;
        let mut high_pulse_count = 0;
        for sig in &self.pending_pulses {
            match sig.pulse {
                Pulse::HIGH => high_pulse_count += 1,
                Pulse::LOW => low_pulse_count += 1,
            }
        }

        self.pending_pulses = next_pending_pulses;
        return (low_pulse_count, high_pulse_count);
    }

    fn process_signals(&mut self, pending_pulses: Vec<Signal>) -> Vec<Signal> {
        match self.state_cache.get(self) {
            Some((new_state, output_pulses)) => {
                self.modules = new_state.modules;
                return output_pulses.clone();
            }
            None => {}
        }

        let mut next_pending_pulses = Vec::new();
        for pending_pulse in &pending_pulses {
            match self.modules.get_mut(&pending_pulse.destination) {
                Some(module) => next_pending_pulses
                    .extend(module.receive(pending_pulse.pulse, &pending_pulse.source)),
                None => {}
            };
        }

        self.state_cache
            .insert(self.clone(), next_pending_pulses.clone());
        return next_pending_pulses;
    }
    fn push_button(&mut self) -> (usize, usize) {
        self.pending_pulses.extend(
            self.modules
                .get_mut("broadcaster")
                .unwrap()
                .receive(Pulse::LOW, &"my finger".to_string()),
        );

        let mut low_total_pulses = 1;
        let mut high_total_pulses = 0;

        loop {
            let (low_pulses, high_pulses) = self.tick();
            low_total_pulses += low_pulses;
            high_total_pulses += high_pulses;

            if low_pulses + high_pulses == 0 {
                break;
            }
        }
        return (low_total_pulses, high_total_pulses);
    }
}

struct ModuleBase {
    module_id: String,
    destination_modules: Vec<String>,
}

impl ModuleBase {
    fn forward(&self, pulse: Pulse) -> Vec<Signal> {
        self.destination_modules
            .iter()
            .map(|destination| Signal {
                source: self.module_id.clone(),
                destination: destination.clone(),
                pulse,
            })
            .collect()
    }

    fn new(module_id: &String, destinations: Vec<String>) -> ModuleBase {
        ModuleBase {
            module_id: module_id.clone(),
            destination_modules: destinations.clone(),
        }
    }

    fn get_destinations(&self) -> Vec<String> {
        self.destination_modules.clone()
    }
}

trait Module {
    fn receive(&mut self, pulse: Pulse, input: &String) -> Vec<Signal>;

    fn add_input(&mut self, input: &String);

    fn get_destinations(&self) -> Vec<String>;

    fn my_hash(&self, s: &mut DefaultHasher);
}

struct FlipFlop {
    state: Pulse,
    base: ModuleBase,
}

struct Conjunction {
    last_pulses: std::collections::HashMap<String, Pulse>,
    base: ModuleBase,
}

struct Broadcast {
    base: ModuleBase,
}

impl Module for Broadcast {
    fn receive(&mut self, pulse: Pulse, _input: &String) -> Vec<Signal> {
        self.base.forward(pulse)
    }

    fn add_input(&mut self, _input: &String) {}
    fn get_destinations(&self) -> Vec<String> {
        self.base.get_destinations()
    }

    fn my_hash(&self, s: &mut DefaultHasher) {
        self.base.module_id.hash(s);
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, pulse: Pulse, _input: &String) -> Vec<Signal> {
        if pulse == Pulse::LOW {
            self.state = self.state.inv();
            return self.base.forward(self.state);
        }
        return Vec::new();
    }
    fn add_input(&mut self, _input: &String) {}
    fn get_destinations(&self) -> Vec<String> {
        self.base.get_destinations()
    }

    fn my_hash(&self, s: &mut DefaultHasher) {
        self.base.module_id.hash(s);
        self.state.hash(s);
    }
}

impl Module for Conjunction {
    fn receive(&mut self, pulse: Pulse, input: &String) -> Vec<Signal> {
        self.last_pulses.insert(input.clone(), pulse);

        for input_state in self.last_pulses.values() {
            if *input_state == Pulse::LOW {
                return self.base.forward(Pulse::HIGH);
            }
        }
        return self.base.forward(Pulse::LOW);
    }

    fn add_input(&mut self, input: &String) {
        self.last_pulses.insert(input.clone(), Pulse::LOW);
    }
    fn get_destinations(&self) -> Vec<String> {
        self.base.get_destinations()
    }

    fn my_hash(&self, s: &mut DefaultHasher) {
        self.base.module_id.hash(s);

        let mut inputs = self.last_pulses.keys().collect::<Vec<_>>();
        inputs.sort_unstable();

        for input in inputs {
            self.last_pulses.get(input).unwrap().hash(s);
        }
    }
}

impl Broadcast {
    fn new(base: ModuleBase) -> Broadcast {
        Broadcast { base }
    }
}

impl FlipFlop {
    fn new(base: ModuleBase) -> FlipFlop {
        FlipFlop {
            base,
            state: Pulse::LOW,
        }
    }
}

impl Conjunction {
    fn new(base: ModuleBase) -> Conjunction {
        Conjunction {
            base,
            last_pulses: std::collections::HashMap::new(),
        }
    }
}

#[aoc(day20, part1)]
fn day20part1(input: &str) -> usize {
    // let input = "broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a";

    // let input = "broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output";

    let mut machine = Machine::from_str(input);
    let mut total_low_pulses = 0;
    let mut total_high_pulses = 0;

    for _ in 0..1000 {
        let (low_pulses, high_pulses) = machine.push_button();
        total_low_pulses += low_pulses;
        total_high_pulses += high_pulses;
    }

    let ans = total_low_pulses * total_high_pulses;
    return ans;
}
