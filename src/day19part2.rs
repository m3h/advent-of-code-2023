use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rating {
    min: i64,
    max: i64,
}

impl Rating {
    fn new() -> Rating {
        Rating { min: 1, max: 4000 }
    }
}

#[derive(Debug, Clone)]
struct Part {
    ratings: std::collections::HashMap<String, Rating>,
}

impl Part {
    fn new() -> Part {
        Part {
            ratings: std::collections::HashMap::from([
                ("x".to_string(), Rating::new()),
                ("m".to_string(), Rating::new()),
                ("a".to_string(), Rating::new()),
                ("s".to_string(), Rating::new()),
            ]),
        }
    }
    fn new_failing() -> Part {
        let failing_rating = Rating { min: 1, max: 0 };

        return Part {
            ratings: std::collections::HashMap::from([
                ("x".to_string(), failing_rating),
                ("m".to_string(), failing_rating),
                ("a".to_string(), failing_rating),
                ("s".to_string(), failing_rating),
            ]),
        };
    }

    fn combinations(&self) -> i64 {
        let mut total = 1;
        for rating_type in ["x", "m", "a", "s"] {
            let r = self.ratings.get(rating_type).unwrap();
            total *= r.max - r.min + 1;
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
        let re_always_true = Regex::new(r"^(?<next_workflow>[a-zA-Z]+)$").unwrap();
        let re_less_than =
            Regex::new(r"^(?<lhs>[a-zA-Z]+)<(?<rhs>\d+):(?<next_workflow>[a-zA-Z]+)$").unwrap();
        let re_greater_than =
            Regex::new(r"^(?<lhs>[a-zA-Z]+)>(?<rhs>\d+):(?<next_workflow>[a-zA-Z]+)$").unwrap();

        match re_always_true.captures(instruction_str) {
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

        match re_less_than.captures(instruction_str) {
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

        match re_greater_than.captures(instruction_str) {
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

    fn test(&self, part: &Part) -> (Part, Part, String) {
        match self.operation {
            WorkflowOperation::AlwaysTrue => (
                part.clone(),
                Part::new_failing(),
                self.next_workflow.clone(),
            ),
            WorkflowOperation::GreaterThan => {
                let mut passing = part.clone();
                let mut failing = part.clone();

                // new min to pass
                let passing_rating = &mut passing.ratings.get_mut(&self.lhs).unwrap();
                passing_rating.min = passing_rating.min.max(self.rhs + 1);

                let failing_rating = &mut failing.ratings.get_mut(&self.lhs).unwrap();
                failing_rating.max = failing_rating.max.min(self.rhs);

                return (passing, failing, self.next_workflow.clone());
            }
            WorkflowOperation::LessThan => {
                let mut passing = part.clone();
                let mut failing = part.clone();

                // new max to pass
                let passing_rating = &mut passing.ratings.get_mut(&self.lhs).unwrap();
                passing_rating.max = passing_rating.max.min(self.rhs - 1);

                let failing_rating = &mut failing.ratings.get_mut(&self.lhs).unwrap();
                failing_rating.min = failing_rating.min.max(self.rhs);

                return (passing, failing, self.next_workflow.clone());
            }
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

    fn next(&self, part: &Part) -> Vec<(String, Part)> {
        let mut workflow_part_pairs = Vec::new();

        let mut remaining_part = part.clone();
        for workflow_instruction in &self.instructions {
            let (passing_part, failing_part, next_workflow) =
                workflow_instruction.test(&remaining_part);
            workflow_part_pairs.push((next_workflow, passing_part));
            remaining_part = failing_part;
        }
        assert!(remaining_part.combinations() == 0);
        return workflow_part_pairs;
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

        for line in workflows_input.lines() {
            let line = line.trim();

            let re = Regex::new(r"(?<id>[a-z]+)\{(?<instructions>.*)\}").unwrap();
            let caps = re.captures(line).unwrap();

            let id = &caps["id"];
            let instructions = &caps["instructions"];

            workflows
                .workflows
                .insert(id.to_string(), Workflow::from_str(instructions));
        }

        return workflows;
    }
    fn final_rating(&self, workflow: &String, part: &Part) -> Vec<Part> {
        if workflow == "A" {
            return vec![part.clone()];
        } else if workflow == "R" {
            return vec![];
        } else {
            let next_paths = self.workflows.get(workflow).unwrap().next(part);

            let mut ret = vec![];
            for (next_workflow, next_path) in &next_paths {
                let r = self.final_rating(next_workflow, next_path);
                ret.extend(r);
            }
            return ret;
        }
    }

    fn count_combinations(&self) -> i64 {
        let mut combinations = 0;
        let p = Part::new();

        let accepted_parts = self.final_rating(&"in".to_string(), &p);

        for part in &accepted_parts {
            combinations += part.combinations();
        }

        return combinations;
    }
}

#[aoc(day19, part2)]
fn day19part2(input: &str) -> i64 {
    let input_parts: Vec<_> = input.trim().split("\n\n").collect();
    assert!(input_parts.len() == 2);

    let workflows = Workflows::from_str(input_parts[0]);

    return workflows.count_combinations();
}
