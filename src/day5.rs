use petgraph::graphmap::DiGraphMap;
use std::fs;

fn parse_input() -> (String, String) {
    let file_str = fs::read_to_string("day5.in").unwrap();

    let (first_part, second_part) = file_str.split_once("\n\n").unwrap();

    (first_part.to_owned(), second_part.trim().to_owned())
}

fn parse_order_into_graph(rules: &str) -> DiGraphMap<usize, ()> {
    let parsed = rules
        .split("\n")
        .map(|x| x.split_once("|").unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));

    let mut g = DiGraphMap::new();

    for (source, dest) in parsed {
        if !g.contains_node(source) {
            g.add_node(source);
        }
        if !g.contains_node(dest) {
            g.add_node(dest);
        }

        g.add_edge(source, dest, ());
    }

    g
}

fn line_is_okay(line: &Vec<usize>, topo: &Vec<usize>) -> bool {
    let topo_filter: Vec<&usize> = topo.iter().filter(|x| line.contains(x)).collect();

    if line.len() != topo_filter.len() {
        eprintln!("error");
        return false;
    }

    for i in 0..line.len() {
        if line[i] != *topo_filter[i] {
            return false;
        }
    }

    true
}

pub fn task1() -> usize {
    let (first_part, second_part) = parse_input();
    let graph = parse_order_into_graph(&first_part);

    let mut result = 0;
    let mut curr_graph: DiGraphMap<usize, ()> = DiGraphMap::new();
    for line in second_part.split("\n") {
        let line_vec: Vec<usize> = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        for node in &line_vec {
            if graph.contains_node(*node) {
                curr_graph.add_node(*node);
                for (source, dest, _) in
                    graph.all_edges().filter(|(source, _, &_)| *source == *node)
                {
                    curr_graph.add_edge(source, dest, ());
                }
            }
        }

        if line_is_okay(
            &line_vec,
            &petgraph::algo::toposort(&curr_graph, None).unwrap(),
        ) {
            result += line_vec[line_vec.len() / 2];
        }
        curr_graph.clear();
    }

    result
}

pub fn task2() -> usize {
    let (first_part, second_part) = parse_input();
    let graph = parse_order_into_graph(&first_part);

    let mut result = 0;
    let mut curr_graph: DiGraphMap<usize, ()> = DiGraphMap::new();
    for line in second_part.split("\n") {
        let line_vec: Vec<usize> = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        for node in &line_vec {
            if graph.contains_node(*node) {
                curr_graph.add_node(*node);
                for (source, dest, _) in graph
                    .all_edges()
                    .filter(|(source, dest, &_)| *source == *node && line_vec.contains(dest))
                {
                    curr_graph.add_edge(source, dest, ());
                }
            }
        }

        let curr_topo = petgraph::algo::toposort(&curr_graph, None).unwrap();
        if !line_is_okay(&line_vec, &curr_topo) {
            result += curr_topo[curr_topo.len() / 2];
        }

        curr_graph.clear();
    }

    result
}
