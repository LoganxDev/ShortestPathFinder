extern crate fnv;

use std::env;
use std::fs::File;
use fnv::FnvHashMap;
use std::io::{BufReader,BufRead};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    // Grab text file name from command line
    let args: Vec<String> = env::args().collect();

    let nodes: Vec<(i32, i32)> = set_nodes(&args);
    let graph: FnvHashMap<i32, Vec<i32>> = set_neighbors(&nodes);

    find_shortest_path(nodes, graph, 223, 8);

	// fmt.Printf("%v\n", path)
    // println!("path: {:?}", end_path);
	// fmt.Printf("Lowest Cost: [%d]\n", shortest_distance[end]+nodes[start])
	// let index = match unvisited.iter().position(|&x| x.1 == start) {
	// 	Some(i) => i,
	// 	None => println!("Helloooo"),
	// };
    // println!("lowest cost: {}", shortest_distance[&end] + nodes[index].1);

    let elapsed = start.elapsed();
    println!("Elapsed: {} ns", elapsed.subsec_nanos());
    println!("Elapsed: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

// Turn text file into a vector of nodes (tuples) where node.0 is the cost and node.1 is the positon
fn set_nodes(args: &[String]) -> Vec<(i32, i32)> {
    let filename = &args[1];

    let f = File::open(filename).unwrap();
    let mut nodes = Vec::new();
    let file = BufReader::new(&f);

    for line in file.lines() {
        let mut s = line.unwrap();
        let mut split = s.split(" ");
        
        let vec: Vec<&str> = split.collect();
        let cost = vec[1].parse::<i32>().unwrap();
        let position = vec[0].parse::<i32>().unwrap();
		let node = (cost, position);

        nodes.push(node);
    }
    nodes
}

fn set_neighbors(nodes: &Vec<(i32, i32)>) -> FnvHashMap<i32, Vec<i32>> {
    // setup neighbors based on position
	let mut graph = FnvHashMap::default();
    for node in nodes {
		println!("{:?} {:?}", node.0, node.1);
        if node.1 == 1 {
            graph.insert(node.1, vec![node.1 + 8, node.1 + 15]);
        } else if node.1 == 8 {
            graph.insert(node.1, vec![node.1 + 7, node.1 + 15]);
        } else if node.1 == 2 || node.1 == 3 || node.1 == 4 || node.1 == 5 || node.1 == 6 || node.1 == 7 {
            graph.insert(node.1, vec![node.1 + 7, node.1 + 15, node.1 + 8]);
        } else if node.1 == 9 || node.1 == 10 || node.1 == 11 || node.1 == 12 || node.1 == 13 || node.1 == 14 || node.1 == 15 {
            graph.insert(node.1, vec![node.1 - 8, node.1 + 7, node.1 + 15, node.1 + 8, node.1 - 7]);
        } else if node.1 == 226 {
            graph.insert(node.1, vec![node.1 - 15, node.1 - 7]);
        } else if node.1 == 233 {
            graph.insert(node.1, vec![node.1 - 15, node.1 - 8]);
        } else if node.1 == 227 || node.1 == 228 || node.1 == 229 || node.1 == 230 || node.1 == 231 || node.1 == 232 {
            graph.insert(node.1, vec![node.1 - 15, node.1 - 8, node.1 + 7]);
        } else if node.1 == 219 || node.1 == 220 || node.1 == 221 || node.1 == 222 || node.1 == 223 || node.1 == 224 || node.1 == 225 {
            graph.insert(node.1, vec![node.1 - 15, node.1 - 8, node.1 + 7, node.1 + 8, node.1 - 7]);
        } else if node.1 == 16 || node.1 == 31 || node.1 == 46 || node.1 == 61 || node.1 == 76 || node.1 == 91 || node.1 == 106 || node.1 == 121 || node.1 == 136 || node.1 == 151 || node.1 == 166 || node.1 == 181 || node.1 == 196 || node.1 == 211 {
            graph.insert(node.1, vec![node.1 - 15, node.1 + 15, node.1 + 8, node.1 - 7]);
        } else if node.1 == 23 || node.1 == 38 || node.1 == 53 || node.1 == 68 || node.1 == 83 || node.1 == 98 || node.1 == 113 || node.1 == 128 || node.1 == 143 || node.1 == 158 || node.1 == 173 || node.1 == 188 || node.1 == 203 || node.1 == 218 {
            graph.insert(node.1, vec![node.1 - 15, node.1 - 8, node.1 + 15, node.1 + 8, node.1 - 7]);
        } else {
            graph.insert(node.1, vec![node.1 - 15, node.1 - 8, node.1 + 7, node.1 + 15, node.1 + 8, node.1 - 7]);
        }
    }
    graph
}

fn find_shortest_path(nodes: Vec<(i32, i32)>, graph: FnvHashMap<i32, Vec<i32>>, start: i32, end: i32) {
    
    // Copy over nodes into a new map so we don't overwrite values
    let mut unvisited: Vec<(i32, i32)> = nodes.clone();

	let mut shortest_distance: FnvHashMap<i32, i32> = FnvHashMap::default();

	// Map of previous nodes
	let mut previous: FnvHashMap<i32, i32> = FnvHashMap::default();
	
	// First set each node to a large number
	for (k, _v) in &graph {
		shortest_distance.insert(*k, 999);
	}

	// Set first node cost to 0
	shortest_distance.insert(start, 0);

	// while there are still unvisited nodes
	while !(unvisited.is_empty()) {
		let mut min_node = 0;
		let mut i = 0;
		// Find the next low cost node in the map of unvisited tiles
		for node in &unvisited {
			if i == 0 {
				min_node = node.1;
				i = i + 1;
				// If the distance at the node is less than the shortest distance at the
				// minimum node then update the minimum node
			} else if shortest_distance.get(&node.1) < shortest_distance.get(&min_node) {
				min_node = node.1;
			}
		}
		// Loop through the neighbors at the minimum node
		for neighbor in &graph[&min_node] {
			/*
				
			*/
			let index = match unvisited.iter().position(|&x| x.1 == *neighbor) {
				Some(i) => i,
				None => continue,
			};
            let x = &nodes[index];
            let y = shortest_distance[&min_node];
			if (x.1 + y) < shortest_distance[&neighbor] {
                let cost = shortest_distance[&min_node];
				shortest_distance.insert(*neighbor, (x.0 + cost));
				previous.insert(*neighbor, min_node);
			}
		}
		// Delete the minimum node index from map of unvisited tiles
		let index = unvisited.iter().position(|&x| x.1 == min_node).unwrap();
		unvisited.remove(index);
	}

	// Start at the ending node (8) traverse backwards to start node (233)
	let mut path: Vec<i32> = Vec::new();
	let mut current_node = end;
	while current_node != start {
		path.push(current_node);
		current_node = previous[&current_node];
	}
	
	// Create answer path by reversing previous path because we traversed from end to start
	path.push(start);
    let mut end_path = Vec::new();
	for _i in 0..path.len() {
		let j = match path.pop() {
			Some(k) => k,
			None => continue,
		};
		end_path.push(j);
	}
	 println!("path: {:?}", end_path);
}