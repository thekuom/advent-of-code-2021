use std::collections::{HashMap, HashSet};
use aoc::*;

/*
    start
    /   \
c--A-----b--d
    \   /
     end

small caves: c, b, d

1 step: 0 start...........................................................................................................................................................................................
2 step: 0 start-A......................................................................................................................................start-b............................................
3 step: 2 start-A-c...........................................................start-A-b....................................................start-A-end start-b-A.....................start-b-end start-b-d
4 step: 2 start-A-c-A.........................................................start-A-b-d start-A-b-end start-A-b-A......................              start-b-A-c......start-b-A-end
5 step: 2 start-A-c-A-b........................................start-A-c-A-end                          start-A-b-A-c......start-A-b-a-end             start-b-A-c-A....
6 step: 2 start-A-c-A-b-end start-A-c-A-b-d start-A-c-A-b-A....                                         start-A-b-A-c-A....                            start-b-A-c-A-end
7 step: 2                                   start-A-c-A-b-A-end                                         start-A-b-A-c-A-end
*/

fn main() {
    if let Ok(lines) = read_lines("./input/p12.txt") {
        let mut graph = HashMap::new();

        for line in lines {
            let line = line.unwrap();
            let line_vertices = line.split("-").collect::<Vec<_>>();

            let entry1 = graph.entry(line_vertices[0].to_string())
                .or_insert(HashSet::new());
            entry1.insert(line_vertices[1].to_string());

            let entry2 = graph.entry(line_vertices[1].to_string())
                .or_insert(HashSet::new());
            entry2.insert(line_vertices[0].to_string());
        }

        let mut paths = vec![vec!["start".to_string()]];

        let mut step = 0;
        loop {
            let mut new_paths = Vec::new();

            for path in &paths[step] {
                let vertices_in_path = path.split("-").collect::<Vec<_>>();
                let last_vertex = vertices_in_path.last().unwrap();

                if *last_vertex == "end" {
                    continue;
                }

                let any_small_cave_visited_twice = vertices_in_path.iter()
                    .fold(HashMap::new(), |mut acc, v| {
                        if v.to_lowercase() == v.to_string() {
                            let entry = acc.entry(v).or_insert(0);
                            *entry += 1;
                        }

                        acc
                    })
                    .iter()
                    .any(|x| x.1 > &1);

                for edge in graph.get(*last_vertex).unwrap() {
                    let can_visit = edge != "start" // can never visit start again
                        && (
                            edge.to_lowercase() != *edge // can always visit big caves again
                            || (
                                !vertices_in_path.contains(&edge.as_str()) // if the edge has not already been seen
                                    || !any_small_cave_visited_twice) // no small cave has been visited again means we can visit this again if needed
                        );

                    if can_visit {
                        new_paths.push(path.to_owned() + "-" + edge);
                    }
                }
            }

            if new_paths.len() == 0 {
                break;
            }

            paths.push(new_paths.clone());
            step += 1;
        }

        let num_of_paths = paths.iter().fold(0, |acc, item|
            acc + item.iter().filter(|x| x.split("-").last().unwrap() == "end").count()
        );

        println!("{}", num_of_paths);
    }
}
