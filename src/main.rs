use std::collections::{HashMap, hash_map::Entry, BTreeMap};

type Node = usize;
type Cost = i32;

struct WorkQueue(BTreeMap<(Node, Node), Cost>);

impl WorkQueue {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn push_task(&mut self, from: Node, to: Node, new_cost: Cost) {
        let current = self.0.get(&(from, to));
        match current {
            None => {
                self.0.insert((from, to), new_cost);
            },
            Some(current_cost) => {
                if current_cost > &new_cost {
                    self.0.insert((from, to), new_cost);
                }
            },
        }
    }

    fn pop_task(&mut self) -> Option<(Node, Node, Cost)> {
        let first = self.0.keys().next();
        let key = match first {
            Some(key) => ((*key).0, (*key).1),
            None => return None,
        };

        let cost = self.0.remove(&key).unwrap();
        Some((key.0, key.1, cost))
    }
}

struct RoutingInfo {
    minimal_from: Node,
    minimal_cost: Cost,
    paths: HashMap<Node, Cost>,
}

impl RoutingInfo {
    fn new(from: Node, cost: Cost) -> Self {
        let mut paths = HashMap::new();
        paths.insert(from, cost);
        Self {
            minimal_from: from,
            minimal_cost: cost,
            paths,
        }
    }

    fn update_path(&mut self, from: Node, _to: Node, cost: Cost) -> bool {
        let updated = match self.paths.get(&from) {
            Some(old_cost) =>  {
                if *old_cost > cost {
                    self.paths.insert(from, cost);
                    true
                } else {
                    false
                }
            },
            None => {
                self.paths.insert(from, cost);
                true
            }
        };

        if updated && self.minimal_cost > cost {
            self.minimal_from = from;
            self.minimal_cost = cost;
        }

        updated
    }
}

struct Graph {
    destinations_list: HashMap<Node, Vec<(Node, Cost)>>,
    sources_list: HashMap<Node, RoutingInfo>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut destinations_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let sources_list: HashMap<Node, RoutingInfo> = HashMap::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = destinations_list
                .entry(source)
                .or_insert_with(|| Vec::new());

            destinations.push((destination, cost));
        }

        let graph = Graph {
            destinations_list,
            sources_list,
        };

        graph
    }

    fn shortest_path(&mut self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
        let found = self.populate_sources_list(start, goal);

        if found {
            let mut path = Vec::new();
            let mut current = goal;

            loop {
                path.push(current);
                if current == start {
                    break;
                }
                current = self.sources_list.get(&current).unwrap().minimal_from;
            }
        
            path.reverse();
            let cost = self.sources_list.get(&goal).unwrap().minimal_cost;
            Some((path, cost))
        } else {
            None
        }
    }

    fn populate_sources_list(&mut self, start: Node, goal: Node) -> bool {
        let mut queue = WorkQueue::new();

        if let Some(destinations) = self.destinations_list.get(&start) {
            for (to, cost) in destinations {
                queue.push_task(start, *to, *cost);
            }
        }

        let mut found = false;
        while let Some((from, to, new_cost)) = queue.pop_task() {
            if to == goal {
                found = true;
            }

            let entry = self.sources_list.entry(to);
            let update = match entry {
                Entry::Vacant(content) => {
                    let route = RoutingInfo::new(from, new_cost);
                    content.insert(route);
                    true
                },
                Entry::Occupied(mut content) => {
                    let route = content.get_mut();
                    route.update_path(from, to, new_cost)
                },
            };
    
            if update {
                let new_from = to;
                if let Some(edges) = self.destinations_list.get(&new_from) {
                    for (new_to, cost) in edges {
                        if let Entry::Occupied(content) = self.sources_list.entry(*new_to) {
                            let minimal_cost = content.get().minimal_cost;
                            if minimal_cost > cost + new_cost {
                                queue.push_task(new_from, *new_to, cost + new_cost);
                            }
                        } else {
                                queue.push_task(new_from, *new_to, cost + new_cost);
                        }
                    }
                } 
            }
        }

        found
    }
}

fn main() {
    let edge_list = include!("large_graph.in");
    let mut g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = g.shortest_path(1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn empty_graph() {
        let edge_list: Vec<(usize, usize, i32)> = Vec::new();
        let mut g = Graph::from_edge_list(&edge_list);

        let found = g.populate_sources_list(0, 0);

        assert!(!found);
        assert_eq!(g.sources_list.len(), 0);
    }

    #[test]
    fn single_node() {
        let edge_list = vec![(0, 0, 42)];
        let mut g = Graph::from_edge_list(&edge_list);

        let (path, cost) = g.shortest_path(0, 0).unwrap();

        assert_eq!(g.sources_list.len(), 1);
        assert_eq!(g.sources_list.get(&0).unwrap().minimal_from, 0);
        assert_eq!(g.sources_list.get(&0).unwrap().minimal_cost, 42);
        assert_eq!(g.sources_list.get(&0).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&0).unwrap().paths.get(&0).unwrap(), &42);
        assert_eq!(path, vec![0]);
        assert_eq!(cost, 42);
    }

    #[test]
    fn two_nodes_one_edge() {
        let edge_list = vec![(0, 1, 42)];
        let mut g = Graph::from_edge_list(&edge_list);

        let (path, cost) = g.shortest_path(0, 1).unwrap();

        assert_eq!(g.sources_list.len(), 1);
        assert!(g.sources_list.get(&0).is_none());
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_from, 0);
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_cost, 42);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.get(&0).unwrap(), &42);
        assert_eq!(path, vec![0, 1]);
        assert_eq!(cost, 42);
    }

    #[test]
    fn two_nodes_two_edges() {
        let edge_list = vec![(0, 1, 42), (0, 1, 21)];
        let mut g = Graph::from_edge_list(&edge_list);

        let (path, cost) = g.shortest_path(0, 1).unwrap();

        assert_eq!(g.sources_list.len(), 1);
        assert!(g.sources_list.get(&0).is_none());
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_from, 0);
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_cost, 21);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.get(&0).unwrap(), &21);
        assert_eq!(path, vec![0, 1]);
        assert_eq!(cost, 21);
    }

    #[test]
    fn two_destinations() {
        let edge_list = vec![(0, 1, 42), (0, 2, 21)];
        let mut g = Graph::from_edge_list(&edge_list);

        let (path, cost) = g.shortest_path(0, 2).unwrap();

        assert_eq!(g.sources_list.len(), 2);
        assert!(g.sources_list.get(&0).is_none());
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_from, 0);
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_cost, 42);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.get(&0).unwrap(), &42);
        assert_eq!(g.sources_list.get(&2).unwrap().minimal_cost, 21);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.get(&0).unwrap(), &21);
        assert_eq!(path, vec![0, 2]);
        assert_eq!(cost, 21);
    }

    #[test]
    fn two_sources() {
        let edge_list = vec![(0, 2, 42), (1, 2, 21)];
        let mut g = Graph::from_edge_list(&edge_list);

        g.populate_sources_list(0, 2);
        let (path, cost) = g.shortest_path(1, 2).unwrap();

        assert_eq!(g.sources_list.len(), 1);
        assert!(g.sources_list.get(&0).is_none());
        assert_eq!(g.sources_list.get(&2).unwrap().minimal_from, 1);
        assert_eq!(g.sources_list.get(&2).unwrap().minimal_cost, 21);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.len(), 2);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.get(&0).unwrap(), &42);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.get(&1).unwrap(), &21);
        assert_eq!(path, vec![1, 2]);
        assert_eq!(cost, 21);
    }


    #[test]
    fn three_nodes_linear() {
        let edge_list = vec![(0, 1, 42), (1, 2, 21)];
        let mut g = Graph::from_edge_list(&edge_list);

        let (path, cost) = g.shortest_path(0, 2).unwrap();

        assert_eq!(g.sources_list.len(), 2);
        assert!(g.sources_list.get(&0).is_none());
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_cost, 42);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.get(&0).unwrap(), &42);
        assert_eq!(g.sources_list.get(&2).unwrap().minimal_cost, 63);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.get(&1).unwrap(), &63);
        assert_eq!(path, vec![0, 1, 2]);
        assert_eq!(cost, 63);
    }

    #[test]
    fn five_nodes_four_edges() {
        let edge_list = vec![(0, 1, 5), (1, 2, 7), (0, 3, 15), (3, 4, 17)];
        let mut g = Graph::from_edge_list(&edge_list);

        let (path, cost) = g.shortest_path(0, 2).unwrap();

        assert_eq!(g.sources_list.len(), 4);
        assert!(g.sources_list.get(&0).is_none());
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_cost, 5);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.get(&0).unwrap(), &5);
        assert_eq!(g.sources_list.get(&2).unwrap().minimal_cost, 12);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.get(&1).unwrap(), &12);
        assert_eq!(g.sources_list.get(&3).unwrap().minimal_cost, 15);
        assert_eq!(g.sources_list.get(&3).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&3).unwrap().paths.get(&0).unwrap(), &15);
        assert_eq!(g.sources_list.get(&4).unwrap().minimal_cost, 32);
        assert_eq!(g.sources_list.get(&4).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&4).unwrap().paths.get(&3).unwrap(), &32);
        assert_eq!(path, vec![0, 1, 2]);
        assert_eq!(cost, 12);
    }

    #[test]
    fn four_nodes_four_edges() {
        let edge_list = vec![(0, 1, 5), (0, 2, 7), (1, 3, 5), (2, 3, 7)];
        let mut g = Graph::from_edge_list(&edge_list);

        let (path, cost) = g.shortest_path(0, 3).unwrap();

        assert_eq!(g.sources_list.len(), 3);
        assert!(g.sources_list.get(&0).is_none());
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_from, 0);
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_cost, 5);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.get(&0).unwrap(), &5);
        assert_eq!(g.sources_list.get(&2).unwrap().minimal_from, 0);
        assert_eq!(g.sources_list.get(&2).unwrap().minimal_cost, 7);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.get(&0).unwrap(), &7);
        assert_eq!(g.sources_list.get(&3).unwrap().minimal_from, 1);
        assert_eq!(g.sources_list.get(&3).unwrap().minimal_cost, 10);
        assert_eq!(g.sources_list.get(&3).unwrap().paths.len(), 2);
        assert_eq!(g.sources_list.get(&3).unwrap().paths.get(&1).unwrap(), &10);
        assert_eq!(g.sources_list.get(&3).unwrap().paths.get(&2).unwrap(), &14);
        assert_eq!(path, vec![0, 1, 3]);
        assert_eq!(cost, 10);
    }

    #[test]
    fn four_nodes_four_edges_reverse_input() {
        let edge_list = vec![(2, 3, 7), (1, 3, 5), (0, 2, 7), (0, 1, 5)];
        let mut g = Graph::from_edge_list(&edge_list);

        let (path, cost) = g.shortest_path(0, 3).unwrap();

        assert_eq!(g.sources_list.len(), 3);
        assert!(g.sources_list.get(&0).is_none());
        assert_eq!(g.sources_list.get(&1).unwrap().minimal_cost, 5);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&1).unwrap().paths.get(&0).unwrap(), &5);
        assert_eq!(g.sources_list.get(&2).unwrap().minimal_cost, 7);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.len(), 1);
        assert_eq!(g.sources_list.get(&2).unwrap().paths.get(&0).unwrap(), &7);
        assert_eq!(g.sources_list.get(&3).unwrap().minimal_cost, 10);
        assert_eq!(g.sources_list.get(&3).unwrap().paths.len(), 2);
        assert_eq!(g.sources_list.get(&3).unwrap().paths.get(&1).unwrap(), &10);
        assert_eq!(g.sources_list.get(&3).unwrap().paths.get(&2).unwrap(), &14);
        assert_eq!(path, vec![0, 1, 3]);
        assert_eq!(cost, 10);
    }

    #[test]
    #[ignore = "too slow"]
    fn large_graph() {
        let edge_list = include!("large_graph.in");
        let mut g = Graph::from_edge_list(&edge_list);

        let path = g.shortest_path(1000, 9000);

        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 24); 
    }
}
