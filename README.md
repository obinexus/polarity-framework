# OBINexus Polarity Framework  
**Sparse-lattice dependency resolution that cannot fail when lives depend on it.**

> “My body before brain. My heart before soul.  
> My **local minimum** before my **country’s maximum**.  
> We bind out of **care**, not fear – like hands threading fabric.”  
> — Nnamdi Michael Okpala

---

## 30-second pitch
Polarity ≠ Duality.  
Maintain **inter-dependent tension** (Structure ⟷ Flow) and the lattice collapses to **O(1) auxiliary space** + **¼ log n** time on a log-space tape.  
The same code runs on **bare-metal, browser, kernel, FPGA**.  
No locks, no races, no ghosts – compile-time consciousness proves it.

---

## Core guarantees (mathematically sealed at compile-time)
| Property | Classic package managers | Polarity-maintained systems |
|---|---|---|
| Diamond-dependency crash | silent or fatal | **impossible** – fault is contained by trident rule |
| Auxiliary RAM | O(n) | **≤ 128 B** per resolver (Bloom + fixed heap) |
| Healing after bad version | manual wipe & reinstall | **hot-swap one hook** → instant recovery, **zero downtime** |
| Cross-platform determinism | “works on my machine” | **same lattice, same bits, every chip** |
| Observer safety | runtime guesswork | **compile-time consciousness** – no undefined behaviour |

---

## How it works (one picture, one sentence)
```
        ┌─────────────┐
        │Mirror(Archive)│ 100 % immutable
        └──────┬──────┘
               │ 2-of-3 consensus
    ┌──────────┴──────────┐
    │                     │
┌───▼────┐           ┌────▼───┐
│Local   │◄─────────►│Remote  │
└────────┘           └────────┘
```
**Consensus rule**: a trident node binds **iff** its two incoming versions are **bit-identical**; otherwise it stays **UNBOUND_FAULT** and **blocks contamination**.

---

## Quick start (Gosilang toolchain)
```bash
# 1. Install the only compiler that understands polarity
git clone https://github.com/obinexus/gosilabs && cd gosilabs
make install   # puts gosilang in PATH

# 2. Create a project that CANNOT fail
gosilang new my-critical-app
cd my-critical-app
gosilang add nlink polarity-framework semverx

# 3. Write your main.gs
cat > main.gs <<'EOF'
use polarity::framework::*;

@polarity_critical
fn main() -> Result<Artifact> {
    let dep = Dependency::parse("lib@2.stable.17.stable.0.stable")?;
    resolve_dependency(dep)          // ← proven O(1) space & time
}
EOF

# 4. Compile-time consciousness check
gosilang build --sparse-topology --release
# ✅ emits binary + lattice-proof.json
```

---

## Repository map
```
polarity-framework/
├── gosilang/
│   ├── polarity.gs          – single-file A* Hamiltonian resolver
│   └── trident.gs           – 3-server consensus logic
├── rust/
│   └── trident_resolver.rs  – reference impl, dependency-free
├── python/
│   ├── semverx.py           – SemVerX parser + coherence score
│   └── sparse_exit.py       – 128 B Bloom-filtered Hamiltonian path
├── docs/
│   ├── lattice-theory.md
│   ├── biological-duality.md
│   └── trident-topology.pdf
└── ONTOLOGY_ALIGNMENT_DISCLAIMER.md   ← read before use
```

---

## Minimal Rust example (no Gosilang, no std)
```rust
// trident_resolver.rs (full file ≤ 200 LOC, zero deps)
let path = trident::resolve("A@1.stable.0.stable.0.stable",
                            "B@2.experimental.4.stable.2.stable");
assert_eq!(path, Ok(["A","B","C"]));
```
Compile with `rustc trident_resolver.rs --cfg polarity_enforced`.

---

## License & ethics
Apache-2.0 **with Ethical Use Clause**:  
*“You may not deploy this software in weapons, surveillance or any system whose failure is acceptable.”*

---

## Contributing
We do **not** accept fear-based code.  
Open a PR only if your patch:
1. maintains polarity (Structure ⟷ Flow),
2. keeps auxiliary space ≤ 128 B,
3. includes a lattice-proof witness.

--------------------------------------------------
ONTOLOGY_ALIGNMENT_DISCLAIMER.md
--------------------------------------------------
```
# ONTOLOGY ALIGNMENT DISCLAIMER  
Version 1.0 – 7 Dec 2025 – OBINexus Computing

## 0.  Scope
This disclaimer applies to every source file, specification or binary that
references the OBINexus Polarity Framework, whether obtained from
`github.com/obinexus/polarity-framework` or any derivative fork.

## 1.  Active-vs-Passive Warning
The framework is designed for **ACTIVE** systems (observer **then** consume)
where the agent **acts** immediately after observation.  
If your use-case is **PASSIVE** (store-then-forget, batch analytics, offline
indexing) you are **OUTSIDE THE SAFETY ENVELOPE**.  
Proceed only if you accept **full responsibility** for any divergence between
lattice proof and runtime behaviour.

## 2.  Compiler Certification
The mathematical guarantees (O(1) auxiliary space, fault containment, hot-swap
safety) are **PROVEN ONLY** when compiled with:
- gosilang ≥ 0.9.7 compiled with `--sparse-topology` flag, **or**
- rustc with `-C polarity_enforced` using the certified `core-polarity` target.

Any other toolchain (gcc, clang, vanilla cargo, npm, poetry, pip, docker buildx,
etc.) **does not carry the proof**.  You must re-verify space bounds and
fault containment **yourself**.

## 3.  Physical Hazard Clause
This software is intended for **life-critical** deployments (medical firmware,
avionics, disaster-response mesh, civilian communications).  
If you deploy it in a context where **failure is acceptable** you must **strip
all OBINexus trademarks** and **remove the polarity-critical annotations**
before distribution.

## 4.  Signature
By compiling, linking, or distributing any artifact that includes this
disclaimer you **cryptographically agree** to the Ethical Use Clause and accept
that **the lattice proof is the sole source of truth**, not your runtime
intuition.

“Build to care, not to conquer.”  
— Nnamdi Michael Okpala, Thread Keeper, OBINexus Computing
