use lazy_static::lazy_static;

use regex::Regex;

struct Part {
    ratings: std::collections::HashMap<String, i64>,
}

impl Part {
    fn from_str(part_str: &str) -> Part {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?<rating_type>[a-z])=(?<rating_value>\d+)").unwrap();
        }

        let ratings: Vec<(String, i64)> = RE
            .captures_iter(part_str)
            .map(|c| {
                (
                    c["rating_type"].to_string(),
                    c["rating_value"].parse().unwrap(),
                )
            })
            .collect();

        let mut part = Part {
            ratings: std::collections::HashMap::new(),
        };
        for (rating_type, rating_value) in &ratings {
            part.ratings.insert(rating_type.clone(), *rating_value);
        }
        return part;
    }

    fn total_rating(&self) -> i64 {
        let mut total = 0;
        for (_, rating) in &self.ratings {
            total += *rating;
        }
        return total;
    }
}

enum WorkflowOperation {
    LessThan,
    GreaterThan,
    AlwaysTrue,
}

struct WorkflowInstruction {
    lhs: String,
    operation: WorkflowOperation,
    rhs: i64,
    next_workflow: String,
}

impl WorkflowInstruction {
    fn from_str(instruction_str: &str) -> WorkflowInstruction {
        lazy_static! {
            static ref RE_ALWAYS_TRUE: Regex =
                Regex::new(r"^(?<next_workflow>[a-zA-Z]+)$").unwrap();
            static ref RE_LESS_THAN: Regex =
                Regex::new(r"^(?<lhs>[a-zA-Z]+)<(?<rhs>\d+):(?<next_workflow>[a-zA-Z]+)$").unwrap();
            static ref RE_GREATER_THAN: Regex =
                Regex::new(r"^(?<lhs>[a-zA-Z]+)>(?<rhs>\d+):(?<next_workflow>[a-zA-Z]+)$").unwrap();
        }

        match RE_ALWAYS_TRUE.captures(instruction_str) {
            Some(caps) => {
                return WorkflowInstruction {
                    lhs: String::new(),
                    operation: WorkflowOperation::AlwaysTrue,
                    rhs: 0,
                    next_workflow: caps["next_workflow"].to_string(),
                }
            }
            None => {}
        };

        match RE_LESS_THAN.captures(instruction_str) {
            Some(caps) => {
                return WorkflowInstruction {
                    lhs: caps["lhs"].to_string(),
                    operation: WorkflowOperation::LessThan,
                    rhs: caps["rhs"].parse().unwrap(),
                    next_workflow: caps["next_workflow"].to_string(),
                }
            }
            None => {}
        }

        match RE_GREATER_THAN.captures(instruction_str) {
            Some(caps) => {
                return WorkflowInstruction {
                    lhs: caps["lhs"].to_string(),
                    operation: WorkflowOperation::GreaterThan,
                    rhs: caps["rhs"].parse().unwrap(),
                    next_workflow: caps["next_workflow"].to_string(),
                }
            }
            None => {}
        }

        panic!("no matching workflow instruction!");
    }
    fn next(&self, part: &Part) -> Option<String> {
        let part_rating = match part.ratings.get(&self.lhs) {
            Some(r) => *r,
            None => -1,
        };

        let return_next_workflow = match self.operation {
            WorkflowOperation::AlwaysTrue => true,
            WorkflowOperation::GreaterThan => part_rating > self.rhs,
            WorkflowOperation::LessThan => part_rating < self.rhs,
        };

        if return_next_workflow {
            return Some(self.next_workflow.clone());
        } else {
            return None;
        }
    }
}

struct Workflow {
    instructions: Vec<WorkflowInstruction>,
}

impl Workflow {
    fn from_str(instructions_str: &str) -> Workflow {
        let mut workflow = Workflow {
            instructions: Vec::new(),
        };

        let mut unprocessed = &instructions_str[..];
        while unprocessed.len() > 0 {
            let i = match unprocessed.find(',') {
                Some(i) => i,
                None => unprocessed.len(),
            };

            workflow
                .instructions
                .push(WorkflowInstruction::from_str(&unprocessed[..i]));
            unprocessed = &unprocessed[(i + 1).min(unprocessed.len())..];
        }
        return workflow;
    }

    fn next(&self, part: &Part) -> String {
        for workflow_instruction in &self.instructions {
            let next_workflow = workflow_instruction.next(part);
            match next_workflow {
                Some(s) => return s,
                None => continue,
            }
        }
        panic!("no matching workflow instruction!");
    }
}

struct Workflows {
    workflows: std::collections::HashMap<String, Workflow>,
}

impl Workflows {
    fn from_str(workflows_input: &str) -> Workflows {
        let mut workflows = Workflows {
            workflows: std::collections::HashMap::new(),
        };

        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?<id>[a-z]+)\{(?<instructions>.*)\}").unwrap();
        }
        for line in workflows_input.lines() {
            let line = line.trim();

            let caps = RE.captures(line).unwrap();

            let id = &caps["id"];
            let instructions = &caps["instructions"];

            workflows
                .workflows
                .insert(id.to_string(), Workflow::from_str(instructions));
        }

        return workflows;
    }
    fn final_rating(&self, part: &Part) -> i64 {
        let mut current_workflow = "in".to_string();

        while current_workflow != "A" && current_workflow != "R" {
            current_workflow = self.workflows.get(&current_workflow).unwrap().next(part);
        }

        return if current_workflow == "A" {
            part.total_rating()
        } else if current_workflow == "R" {
            0
        } else {
            panic!("oh no!");
        };
    }
}

#[aoc(day19, part1)]
fn day19part1(input: &str) -> i64 {
    let input_parts: Vec<_> = input.trim().split("\n\n").collect();
    assert!(input_parts.len() == 2);

    let workflows: Workflows = Workflows::from_str(input_parts[0]);

    let mut total = 0;
    for part_line in input_parts[1].lines() {
        let part_line = part_line.trim();

        let part = Part::from_str(part_line);
        total += workflows.final_rating(&part);
    }
    return total;
}
