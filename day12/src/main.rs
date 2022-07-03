use std::{collections::HashMap, fs};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn make_graph(data: &str) -> Graph {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in data.lines() {
        let (from, to) = line.split_once('-').unwrap();
        map.entry(from).or_default().push(to);
        map.entry(to).or_default().push(from);
    }

    map
}

fn dfs<'a>(graph: &'a Graph) -> usize {
    #[derive(Default, Clone, Debug)]
    struct State<'b> {
        cur: &'b str,
        visited: im::HashSet<&'b str>,
        second_visit_used: bool,
        path: im::Vector<&'b str>,
    }

    let mut paths = std::collections::HashSet::new();

    let start_state = State {
        cur: "start",
        ..Default::default()
    };
    let mut to_visit = vec![start_state];

    while let Some(state) = to_visit.pop() {
        let mut state = state.clone();
        state.path.push_back(state.cur);
        let mut explore_neighbors = |state: State<'a>| {
            for neighbor in graph.get(state.cur).unwrap() {
                if !state.visited.contains(neighbor) {
                    to_visit.push(State {
                        cur: neighbor,
                        ..state.clone()
                    });
                }
            }
        };

        if state.cur == "end" {
            paths.insert(state.path.clone());
        } else if state.cur.chars().all(char::is_lowercase) {
            if !state.second_visit_used && state.cur != "start" {
                // once with the second state being used
                let used_second_state = State {
                    second_visit_used: true,
                    ..state.clone()
                };
                explore_neighbors(used_second_state);
            }

            // once with a standard visit
            let visited = state.visited.update(state.cur);
            let new_state = State { visited, ..state };
            explore_neighbors(new_state);
        } else {
            explore_neighbors(state);
        }
    }

    paths.len()
}

fn main() {
    let data = fs::read_to_string("day12/day12.txt").unwrap();
    let graph = make_graph(&data);

    let score = dfs(&graph);
    println!("Answer: {}", score);
}
