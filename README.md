# ternary-geometry

Geometric algorithms for ternary spaces — points, distances (Manhattan, Lee, Hamming), Voronoi diagrams, convex hulls, and area/volume on the `{0, 1, 2}` grid.

## Why This Exists

The ternary grid — where each coordinate takes one of three values — is the natural geometry for ternary computing. With only 9 points in 2D and 27 in 3D, you can enumerate and reason about the entire space exhaustively. This crate provides geometric primitives that respect the discrete, cyclic nature of ternary coordinates, including Lee distance (cyclic distance on ℤ/3ℤ) which has no analog in standard Euclidean geometry.

## Core Concepts

- **TernaryPoint** — A point in {0, 1, 2}³ with 2D and 3D variants
- **Manhattan Distance** — Standard L1 distance between ternary points
- **Lee Distance** — Cyclic distance on ℤ/3ℤ (wraps around: distance 0→2 is 1, not 2)
- **Hamming Distance** — Count of differing coordinates
- **Voronoi Diagrams** — Nearest-seed partitioning with Manhattan or Lee distance
- **Convex Hull** — Gift-wrapping algorithm for 2D ternary point sets
- **Polygon Area** — Shoelace formula for 2D ternary polygons

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-geometry = "0.1"
```

```rust
use ternary_geometry::*;

// Create points
let a = TernaryPoint::new_2d(0, 0);
let b = TernaryPoint::new_2d(2, 2);
let c = TernaryPoint::new(0, 1, 2);

// Distance metrics
println!("Manhattan: {}", manhattan_distance(&a, &b));  // 4
println!("Lee: {}", lee_distance(&a, &b));              // 2 (wraps)
println!("Hamming: {}", hamming_distance(&a, &b));      // 2

// All points in the grid
let all_2d = TernaryPoint::all_2d();  // 9 points
let all_3d = TernaryPoint::all_3d();  // 27 points
println!("2D grid: {} points", all_2d.len());
println!("3D grid: {} points", all_3d.len());

// Voronoi diagrams
let seeds = vec![TernaryPoint::new_2d(0, 0), TernaryPoint::new_2d(2, 2)];
let regions = voronoi_manhattan_2d(&seeds);
for (point, seed_idx) in &regions {
    println!("{:?} → seed {}", point.coords, seed_idx);
}

// Convex hull
let pts = vec![
    TernaryPoint::new_2d(0, 0),
    TernaryPoint::new_2d(2, 0),
    TernaryPoint::new_2d(2, 2),
    TernaryPoint::new_2d(0, 2),
    TernaryPoint::new_2d(1, 1),  // interior
];
let hull = convex_hull_2d(&pts);
println!("Hull indices: {:?}", hull);

// Polygon area (shoelace formula)
let area = polygon_area_2d(&pts[..4].to_vec());
println!("Square area: {}", area);  // 4.0

// Bounding box and centroid
let bbox = bounding_box_2d(&pts);
let (cx, cy) = centroid_2d(&pts);
println!("BBox: {:?}", bbox);
println!("Centroid: ({}, {})", cx, cy);

// Line operations
let line = TernaryLine::new(a, b);
println!("Midpoint: {:?}", line.midpoint().coords);
println!("On segment: {}", point_on_segment(&TernaryPoint::new_2d(1, 1), &line));
```

## API Overview

| Type / Function | Description |
|---|---|
| `TernaryPoint` | Point in {0,1,2}³ with `all_2d()`, `all_3d()` |
| `TernaryLine` | Line segment with `midpoint`, `manhattan_length`, `lee_length` |
| `manhattan_distance` / `lee_distance` / `hamming_distance` | Distance metrics |
| `voronoi_manhattan_2d/3d` / `voronoi_lee_3d` | Voronoi partitioning |
| `convex_hull_2d` | Gift-wrapping hull computation |
| `polygon_area_2d` | Shoelace formula area |
| `bounding_box_2d` / `points_in_bbox` | Bounding box utilities |
| `centroid_2d` | Mean coordinate |
| `point_on_segment` | Point-line containment test |
| `ternary_grid_volume` / `ternary_grid_area` | Constants (27 and 9) |

## How It Works

**Lee Distance** treats each coordinate as a cyclic value in ℤ/3ℤ. The distance between 0 and 2 is `min(2, 3−2) = 1`, reflecting that 2 "wraps around" to be close to 0. This is the natural metric for ternary arithmetic.

**Voronoi Diagrams** enumerate all grid points and assign each to its nearest seed. With only 9 (2D) or 27 (3D) points, brute-force is efficient and exact. Supports both Manhattan and Lee distance metrics.

**Convex Hull** uses gift wrapping (Jarvis march), choosing the most counterclockwise point at each step. Ties are broken by distance. Interior points are naturally excluded.

**Polygon Area** applies the shoelace formula: `Area = ½|Σ(xᵢyᵢ₊₁ − xᵢ₊₁yᵢ)|`.

## Use Cases

1. **Ternary code analysis** — Compute distance distributions between codewords in ternary error-correcting codes using Lee distance
2. **Game board geometry** — Model small discrete game boards where positions are ternary coordinates
3. **Territory partitioning** — Assign ternary grid cells to nearest facilities using Voronoi diagrams
4. **Pattern recognition** — Measure similarity between ternary patterns with appropriate distance metrics

## Known Limitations

- **Coordinate space is {0, 1, 2}, not {-1, 0, +1}**: Unlike most ternary crates in this ecosystem that use the balanced ternary {-1, 0, +1} representation, this crate uses non-negative coordinates {0, 1, 2}. Converting between this crate's `TernaryPoint` and the balanced ternary used by other crates requires a manual offset (`x - 1`).

- **Grid exhaustively enumerated — only 9 (2D) or 27 (3D) points**: All algorithms (Voronoi, convex hull) enumerate the full grid, which means they cannot generalize to larger ternary spaces or different coordinate ranges. The crate is fundamentally limited to exactly 3 values per dimension.

- **`convex_hull_2d()` uses gift wrapping on at most 9 points**: The gift-wrapping algorithm runs on the 9-point 2D grid. For this tiny domain, any hull algorithm is instant, but the implementation doesn't exploit the known structure of the ternary grid (e.g., the hull is always a subset of the 8 boundary points).

- **`polygon_area_2d()` uses shoelace formula with integer coordinates**: Since all coordinates are in {0, 1, 2}, polygon areas are always multiples of 0.5. The function returns an `f64` but the result is always `0.0`, `0.5`, `1.0`, `1.5`, etc. — the floating-point precision is unnecessary.

- **`TernaryLine::midpoint()` rounds down**: The midpoint of `(0, 0)` and `(1, 2)` is computed as `(0, 1)` rather than `(0.5, 1.5)` because coordinates are integers. This silently loses the true midpoint for most line segments.

- **Voronoi ties go to the first seed**: When a grid point is equidistant from multiple seeds, it is assigned to the seed with the lowest index. The tie-breaking rule is deterministic but arbitrary — it can produce unintuitive region boundaries.

- **No 3D convex hull or 3D volume computation**: The crate provides 2D convex hull and 2D polygon area but no 3D equivalents. `ternary_grid_volume()` returns the constant 27 and `ternary_grid_area()` returns 9 — they are not computed from input.

## Ecosystem

Part of the **SuperInstance** ternary computing crate family:

- `ternary-compression-v2` — Multi-algorithm ternary compression
- `ternary-hash` — Hashing and fingerprinting for ternary data
- `ternary-pca` — Principal component analysis on ternary values
- `ternary-ga` — Genetic algorithms with ternary genomes
- `ternary-matrix` — Compact ternary matrix operations
- `ternary-reservoir` — Echo state networks with ternary nodes
- `ternary-evolution-advanced` — Advanced evolutionary optimization
- `ternary-causality` — Causal inference for ternary systems
- `ternary-consensus` — Distributed consensus for ternary agents

## License

MIT

## See Also
- **ternary-topology** — related
- **ternary-graph** — related
- **ternary-projection** — related
- **ternary-pca** — related
- **ternary-tensor** — related

