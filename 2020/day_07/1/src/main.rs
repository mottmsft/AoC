use std::{
    collections::HashMap,
    collections::HashSet,
    fs::File,
    io::BufRead,
    io::BufReader,
    path::Path,
};

use regex::Regex;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("Could not parse line"))
       .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    edges: HashMap<String, HashMap<String, usize>>, // Bag -> (Contained Bag, Quantity)
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, source: &str, target: &str, quantity: usize) {
        let entry = self.edges.entry(source.to_string()).or_insert_with(HashMap::new);
        entry.insert(target.to_string(), quantity);
    }

    fn bags_containing_at_least(&self, target_color: &str, min_quantity: usize) -> HashSet<String> {
        let mut result = HashSet::new();
        for (source, contained_bags) in &self.edges {
            if contains_at_least(contained_bags, target_color, min_quantity, &self.edges) {
                result.insert(source.clone());
            }
        }
        result
    }
}

fn contains_at_least(
    contained_bags: &HashMap<String, usize>,
    target_color: &str,
    min_quantity: usize,
    graph: &HashMap<String, HashMap<String, usize>>,
) -> bool {
    for (color, quantity) in contained_bags {
        if color == target_color && *quantity >= min_quantity {
            return true;
        }
        if let Some(next_contained_bags) = graph.get(color) {
            if contains_at_least(next_contained_bags, target_color, min_quantity, graph) {
                return true;
            }
        }
    }
    false
}


fn main() {
    let lines = lines_from_file("input.txt");

    let mut graph = Graph::new();

    let re = Regex::new(r"^(?P<color>\w+ \w+) bags contain (?P<contents>.*).$").unwrap();
    let inner_re = Regex::new(r"(?P<quantity>\d+) (?P<color>\w+ \w+) bags?").unwrap();

    for line in lines {
        if let Some(captures) = re.captures(line.as_str()) {
            let main_color = &captures["color"];
            let contents = &captures["contents"];

            println!("Main Color: {}", main_color);

            for inner_cap in inner_re.captures_iter(contents) {
                let quantity: usize = inner_cap["quantity"].parse().unwrap();
                let color = &inner_cap["color"];

                println!("  Quantity: {}, Color: {}", quantity, color);

                graph.add_edge(main_color, color, quantity);
            }
        }
    }

    let bags = graph.bags_containing_at_least("shiny gold", 1);
    let result = bags.len();

    println!("Result: {}", result);
}
