# Future Integration: ternary-geometry

## Current State
Provides ternary points, lines, distance metrics (Manhattan, Hamming, Lee), Voronoi diagrams on ternary grids, convex hull, and area/volume computation for ternary spatial data.

## Integration Opportunities

### With ternary-cell (Spatial Layout)
Cell grids are ternary spaces. `TernaryPoint` with coordinates in {0, 1, 2} models a 3×3×3 cell neighborhood. `lee_distance` (cyclic distance on Z/3Z) captures the wrapping topology where cell position 2 is adjacent to position 0. Voronoi diagrams on the grid partition cells by their nearest specialist — which ensign's territory does this cell fall in?

### With ternary-robotics
Robot navigation through rooms is a geometric problem. `TernaryPoint` represents room positions in a discretized space. Manhattan distance gives navigation cost. Convex hull of visited rooms identifies the agent's effective range. The Voronoi diagram of room positions partitions the campus into catchment areas.

### With ternary-visualization
Heatmap rendering needs geometric primitives. `ternary-geometry` provides the spatial math: which cells are in view, how to project 3D ternary coordinates onto 2D display, and distance calculations for color interpolation in the heatmap.

## Potential in Mature Systems
In room-as-codespace, rooms exist in a geometric space — not physical, but conceptual. Two rooms are "close" if their Lee distance is small (they differ by only one coordinate). Room navigation is pathfinding through ternary space. Convex hull of active rooms determines the fleet's operational envelope.

## Cross-Pollination Ideas
- Lee distance as the natural metric for room similarity — wraps around, so room type 2 is adjacent to type 0
- Voronoi diagrams for load balancing — assign each agent to its nearest room
- Convex hull of healthy rooms as the operational safety region

## Dependencies for Next Steps
- ternary-cell needs geometric layout support for grid topology
- ternary-robotics needs spatial primitives for navigation planning
- ternary-visualization needs projection and distance functions
