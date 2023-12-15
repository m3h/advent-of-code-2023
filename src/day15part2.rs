use std::{collections::HashMap, ops::Index};

fn hash(s: &str) -> usize {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }

    return current_value;
}

struct Lens {
    label: String,
    focal_length: usize,
}

struct LensBox {
    lens_slots: Vec<Lens>,
}

struct Facility {
    boxes: Vec<LensBox>,
}

impl Facility {
    const FACILITY_SIZE: usize = 256;
    fn new() -> Facility {
        let mut boxes = Vec::new();
        for _ in 0..Self::FACILITY_SIZE {
            boxes.push(LensBox {
                lens_slots: Vec::new(),
            })
        }
        return Facility { boxes };
    }

    fn process_input(&mut self, input: &str) -> usize {
        let input: Vec<&str> = input.trim().split(",").collect();

        for step in input {
            self.process_step(step);
        }

        return self.focusing_power();
    }

    fn focusing_power(&self) -> usize {
        let mut sum = 0;
        for box_idx in 0..self.boxes.len() {
            let this_box = &self.boxes[box_idx];
            for slot_number in 0..this_box.lens_slots.len() {
                let lens = &this_box.lens_slots[slot_number];

                let focal_length = lens.focal_length;
                let this_power = (box_idx + 1) * (slot_number + 1) * focal_length;

                sum += this_power;
            }
        }

        return sum;
    }

    fn process_step(&mut self, step: &str) {
        let step: Vec<char> = step.chars().collect();

        for i in 0..step.len() {
            let label: String = step[..i].iter().collect();
            if step[i] == '-' {
                self.process_removal(&label);
            } else if step[i] == '=' {
                let focal_length: String = step[i + 1..].iter().collect();
                self.process_addition(&label, &focal_length)
            }
        }
    }
    fn process_removal(&mut self, label: &str) {
        let h = hash(label);

        let lens_box = &mut self.boxes[h];
        for i in 0..lens_box.lens_slots.len() {
            if lens_box.lens_slots[i].label == label {
                lens_box.lens_slots.remove(i);
                return;
            }
        }
    }

    fn process_addition(&mut self, label: &str, focal_length: &str) {
        let h = hash(label);
        let focal_length = focal_length.parse().unwrap();

        let lens_box = &mut self.boxes[h];
        for i in 0..lens_box.lens_slots.len() {
            if lens_box.lens_slots[i].label == label {
                lens_box.lens_slots[i].focal_length = focal_length;
                return;
            }
        }

        lens_box.lens_slots.push(Lens {
            label: label.to_string(),
            focal_length,
        })
    }
}

#[aoc(day15, part2)]
fn day15part2(input: &str) -> usize {
    let mut facility = Facility::new();
    return facility.process_input(input);
}
