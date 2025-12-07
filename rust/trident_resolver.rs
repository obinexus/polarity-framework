// trident_resolver.rs
// Single-file Rust implementation of a dependency-resolver/scoring system
// described in your notes. This implements:
// 1) Version parsing with optional "channel" (experimental/legacy/lts/stable)
// 2) A consensus-based node scoring from votes (Yes/No/Nil)
// 3) Artifact score influence (artifact confidence / quality)
// 4) A DFS-based traversal that chooses branches by computed score
//
// Usage: compile with `rustc trident_resolver.rs` and run `./semverx_trident_resolver.rs`
// This is intentionally dependency-free (no external crates) so you can run it
// immediately.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Vote { Yes, No, Nil }

#[derive(Debug, Clone)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    channel: String, // e.g. "stable", "experimental", "legacy", "lts"
}

impl Version {
    fn parse(s: &str) -> Self {
        // Expect formats like "1.2.3" or "1.2.3-stable" or "1.2.3-experimental"
        let mut parts = s.splitn(2, '-');
        let nums = parts.next().unwrap_or("0.0.0");
        let channel = parts.next().unwrap_or("stable").to_string();
        let segs: Vec<&str> = nums.split('.').collect();
        let major = segs.get(0).and_then(|p| p.parse().ok()).unwrap_or(0);
        let minor = segs.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);
        let patch = segs.get(2).and_then(|p| p.parse().ok()).unwrap_or(0);
        Version { major, minor, patch, channel }
    }

    fn semver_weight(&self) -> f64 {
        // Simple heuristic: prefer higher major, minor, patch. Normalize to a small weight.
        // This returns a value in (0.0, 1.5] depending on version numbers.
        let base = (self.major as f64) * 100.0 + (self.minor as f64) * 10.0 + (self.patch as f64);
        // compress scale so we don't explode
        let w = (base / (base + 100.0)).min(1.0);
        // channel multiplier: stable and lts > legacy > experimental
        let channel_mult = match self.channel.as_str() {
            "stable" | "lts" => 1.0,
            "legacy" => 0.8,
            "experimental" => 0.6,
            _ => 0.75,
        };
        w * channel_mult
    }
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    version: Version,
    artifact_score: f64, // [0.0, 1.0] confidence/quality of the artifact
    votes: Vec<Vote>,     // consensus votes
}

impl Node {
    fn consensus_score(&self) -> f64 {
        // Map votes -> numeric: Yes=1.0, Nil=0.5, No=0.0
        // Compute average -> in [0.0,1.0]
        if self.votes.is_empty() {
            return 0.5; // neutral by default
        }
        let mut sum = 0.0;
        for v in &self.votes {
            let val = match v {
                Vote::Yes => 1.0,
                Vote::Nil => 0.5,
                Vote::No => 0.0,
            };
            sum += val;
        }
        sum / (self.votes.len() as f64)
    }

    fn combined_score(&self) -> f64 {
        // Combine: consensus_score (50%), artifact_score (30%), semver_weight (20%)
        // Weights chosen as an example—tune to taste.
        let c = self.consensus_score();
        let a = self.artifact_score.clamp(0.0, 1.0);
        let s = self.version.semver_weight().clamp(0.0, 1.0);
        let score = 0.50 * c + 0.30 * a + 0.20 * s;
        score
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Vec<String>>, // adjacency list: id -> children ids
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: HashMap::new(), edges: HashMap::new() }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.edges.entry(from.to_string()).or_default().push(to.to_string());
    }

    // DFS traversal that chooses the best child branch based on combined_score.
    // Returns the chosen path (list of node ids) starting at `start` until a leaf.
    fn resolve_best_path(&self, start: &str) -> Vec<String> {
        let mut path = Vec::new();
        let mut cur = start.to_string();
        let mut seen = HashSet::new();
        while !seen.contains(&cur) {
            seen.insert(cur.clone());
            path.push(cur.clone());
            let children = self.edges.get(&cur);
            match children {
                None => break, // leaf
                Some(list) if list.is_empty() => break,
                Some(list) => {
                    // pick child with highest combined_score; tie-break by semver then id
                    let mut best: Option<(&String, f64)> = None;
                    for cid in list {
                        if let Some(node) = self.nodes.get(cid) {
                            let sc = node.combined_score();
                            match best {
                                None => best = Some((cid, sc)),
                                Some((ref_bid, bsc)) => {
                                    if sc > bsc {
                                        best = Some((cid, sc));
                                    } else if (sc - bsc).abs() < 1e-6 {
                                        // tie-break: semantic version (major,minor,patch)
                                        let curv = &node.version;
                                        let bestv = &self.nodes.get(ref_bid).unwrap().version;
                                        if (curv.major, curv.minor, curv.patch) > (bestv.major, bestv.minor, bestv.patch) {
                                            best = Some((cid, sc));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some((bid, _)) = best {
                        cur = bid.clone();
                        continue;
                    } else {
                        break; // no resolvable child
                    }
                }
            }
        }
        path
    }
}

fn demo_graph() {
    let mut g = Graph::new();

    // Node A
    g.add_node(Node {
        id: "A".to_string(),
        version: Version::parse("1.0.0-stable"),
        artifact_score: 0.95,
        votes: vec![Vote::Yes, Vote::Yes, Vote::Nil],
    });

    // B1 (branch 1)
    g.add_node(Node {
        id: "B1".to_string(),
        version: Version::parse("2.0.0-stable"),
        artifact_score: 0.9,
        votes: vec![Vote::Yes, Vote::Yes, Vote::Yes, Vote::No],
    });

    // B2 (branch 2) - slightly older, more experimental
    g.add_node(Node {
        id: "B2".to_string(),
        version: Version::parse("1.5.2-experimental"),
        artifact_score: 0.7,
        votes: vec![Vote::Yes, Vote::Nil, Vote::No, Vote::No],
    });

    // C depends on whichever B chosen; make two versions of C to show depth
    g.add_node(Node {
        id: "C1".to_string(),
        version: Version::parse("3.0.0-stable"),
        artifact_score: 0.92,
        votes: vec![Vote::Yes, Vote::Yes],
    });

    g.add_node(Node {
        id: "C2".to_string(),
        version: Version::parse("2.9.9-legacy"),
        artifact_score: 0.85,
        votes: vec![Vote::Yes, Vote::Nil],
    });

    // Edges: A -> B1, B2
    g.add_edge("A", "B1");
    g.add_edge("A", "B2");
    // B1 -> C1 ; B2 -> C2
    g.add_edge("B1", "C1");
    g.add_edge("B2", "C2");

    let path = g.resolve_best_path("A");
    println!("Resolved path: {:?}", path);

    // For clarity, print node scores
    for id in &path {
        if let Some(n) = g.nodes.get(id) {
            println!("Node {} -> consensus: {:.3}, artifact: {:.3}, semver: {:.3}, combined: {:.3}",
                     id, n.consensus_score(), n.artifact_score, n.version.semver_weight(), n.combined_score());
        }
    }
}

fn main() {
    println!("Trident-based dependency resolver demo\n");
    demo_graph();
}
