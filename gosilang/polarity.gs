//  polarity.gs
//  Sparse A* with Hamiltonian / Eulerian polarity
//  gosilang 0.9.7+  (github.com/obinexus/gosilabs)

use semverx::SemverKey;
use sparse::{SparseGraph, NodeKey, EdgeKey};
use trident::{Mirror, Local, Remote, Archive, Consensus};

// ---------- compile-time parameters ----------
comptime POLARITY_WINDOW: u16 = 16;      // max |open| – gives O(1) space
comptime HAMILTONIAN_MASK: u64  = 0x_FFFF_FFFF_FFFF_FFFF; // 64-bit Hamiltonian cycle index

// ---------- polarity-aware node ----------
struct PolarityNode {
    key:        NodeKey,
    g:          u32,          // cost from source
    h:          u32,          // admissible heuristic
    f:          u32,          // g + h
    pole:       bool,         // false=Structure, true=Flow
    hamilton:   u64,          // position in Hamiltonian cycle
    euler:      u32,          // Euler tour index
    parent:     NodeKey,
}

// ---------- open-list: fixed-size binary heap ----------
struct PolarityHeap {
    buf: [PolarityNode; POLARITY_WINDOW],
    len: u16,
}
impl PolarityHeap {
    comptime fn new() -> Self { Self{ buf: unsafe{uninit!()}, len:0 } }

    comptime fn push(&mut self, n: PolarityNode) -> bool {
        if self.len == POLARITY_WINDOW { return false; } // drop – keeps O(1)
        let idx = self.len as usize;
        self.buf[idx] = n;
        self.len += 1;
        self.bubble_up(idx);
        true
    }
    comptime fn pop(&mut self) -> Option<PolarityNode> {
        if self.len == 0 { return None; }
        let best = self.buf[0];
        self.len -= 1;
        if self.len > 0 {
            self.buf[0] = self.buf[self.len as usize];
            self.bubble_down(0);
        }
        Some(best)
    }
    // … tiny heap helpers omitted for brevity …
}

// ---------- sparse A* search ----------
comptime fn sparse_polarity_astar(
        graph: &SparseGraph,
        src:   NodeKey,
        dst:   NodeKey,
) -> Option<(u32, ArtifactID)> {
    let mut open   = PolarityHeap::new();
    let mut closed = BitSet64::new();          // constant size – O(1)

    let src_node = PolarityNode{
        key:src, g:0,
        h:euclidean(src,dst),
        f:euclidean(src,dst),
        pole:false,           // start on Structure pole
        hamilton: graph.hamilton_index(src),
        euler:    graph.euler_index(src),
        parent:   NodeKey::NULL,
    };
    open.push(src_node);

    while let Some(curr) = open.pop() {
        if curr.key == dst {
            let artifact = ArtifactID::from_path(&curr);
            return Some((curr.g, artifact));
        }
        closed.set(curr.key);

        // expansion order: Hamiltonian neighbours first, then Euler edges
        let mut next = graph.hamilton_succ(curr.key); // constant time
        if next.is_none() { next = graph.euler_edge(curr.key); }

        while let Some(nkey) = next {
            if closed.is_set(nkey) { next = graph.next_succ(curr.key); continue; }

            let neighbour = PolarityNode{
                key: nkey,
                g: curr.g + 1,
                h: euclidean(nkey, dst),
                f: 0,
                pole: !curr.pole,          // flip polarity
                hamilton: graph.hamilton_index(nkey),
                euler:    graph.euler_index(nkey),
                parent:   curr.key,
            };
            neighbour.f = neighbour.g + neighbour.h;

            // polarity rule: only expand if parent pole ≠ child pole
            if curr.pole != neighbour.pole {
                open.push(neighbour);
            }
            next = graph.next_succ(curr.key);
        }
    }
    None
}

// ---------- public API – compiled into caller ----------
@polarity_critical
pub comptime fn resolve_dependency(dep: Dependency) -> Result<Artifact> {
    let graph = SparseGraph::from_semverx(dep.semver_key());
    let (cost, art_id) = sparse_polarity_astar(&graph, dep.source(), dep.target())
        .ok_or(Error::NoPath)?;

    // trident consensus: 2-of-3 mirrors must agree on ArtifactID
    let local   = Mirror::Local.hash(art_id);
    let remote  = Mirror::Remote.hash(art_id);
    let archive = Mirror::Archive.hash(art_id);

    let consensus = Consensus::two_of_three(local, remote, archive);
    if !consensus.ok() { return Err(Error::TridentFail); }

    // sparse state recovery: materialise from Archive mirror
    Ok(recover_from_mirror(archive))
}

// ---------- helpers ----------
comptime fn euclidean(a: NodeKey, b: NodeKey) -> u32 {
    let (ax,ay) = a.coords();
    let (bx,by) = b.coords();
    let dx = (ax-bx).abs();
    let dy = (ay-by).abs();
    ((dx*dx + dy*dy) as f32).sqrt() as u32
}
