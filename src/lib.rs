#![forbid(unsafe_code)]

//! Geometric algorithms for ternary spaces.
//!
//! Provides ternary points, lines, distance metrics (Manhattan, Hamming, Lee),
//! Voronoi diagrams on ternary grids, convex hull, and area/volume computation.

/// A point in ternary space (each coordinate in {0, 1, 2}).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TernaryPoint {
    pub coords: [u8; 3],
}

impl TernaryPoint {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        assert!(x <= 2 && y <= 2 && z <= 2, "coordinates must be 0, 1, or 2");
        TernaryPoint { coords: [x, y, z] }
    }

    pub fn new_2d(x: u8, y: u8) -> Self {
        Self::new(x, y, 0)
    }

    pub fn x(&self) -> u8 { self.coords[0] }
    pub fn y(&self) -> u8 { self.coords[1] }
    pub fn z(&self) -> u8 { self.coords[2] }

    /// All possible ternary 2D points (9 total).
    pub fn all_2d() -> Vec<Self> {
        let mut pts = Vec::with_capacity(9);
        for x in 0..3u8 {
            for y in 0..3u8 {
                pts.push(TernaryPoint::new_2d(x, y));
            }
        }
        pts
    }

    /// All possible ternary 3D points (27 total).
    pub fn all_3d() -> Vec<Self> {
        let mut pts = Vec::with_capacity(27);
        for x in 0..3u8 {
            for y in 0..3u8 {
                for z in 0..3u8 {
                    pts.push(TernaryPoint::new(x, y, z));
                }
            }
        }
        pts
    }
}

/// Manhattan distance between two ternary points.
pub fn manhattan_distance(a: &TernaryPoint, b: &TernaryPoint) -> u32 {
    a.coords.iter().zip(b.coords.iter())
        .map(|(&x, &y)| (x as i32 - y as i32).unsigned_abs() as u32)
        .sum()
}

/// Lee distance (cyclic distance on Z/3Z).
pub fn lee_distance(a: &TernaryPoint, b: &TernaryPoint) -> u32 {
    a.coords.iter().zip(b.coords.iter())
        .map(|(&x, &y)| {
            let d = (x as i32 - y as i32).unsigned_abs() as u32;
            d.min(3 - d)
        })
        .sum()
}

/// Hamming distance (count of differing coordinates).
pub fn hamming_distance(a: &TernaryPoint, b: &TernaryPoint) -> u32 {
    a.coords.iter().zip(b.coords.iter())
        .filter(|(&x, &y)| x != y)
        .count() as u32
}

/// A line segment in ternary space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TernaryLine {
    pub start: TernaryPoint,
    pub end: TernaryPoint,
}

impl TernaryLine {
    pub fn new(start: TernaryPoint, end: TernaryPoint) -> Self {
        TernaryLine { start, end }
    }

    /// Length in Manhattan distance.
    pub fn manhattan_length(&self) -> u32 {
        manhattan_distance(&self.start, &self.end)
    }

    /// Length in Lee distance.
    pub fn lee_length(&self) -> u32 {
        lee_distance(&self.start, &self.end)
    }

    /// Midpoint (rounded down per coordinate).
    pub fn midpoint(&self) -> TernaryPoint {
        TernaryPoint {
            coords: [
                ((self.start.coords[0] + self.end.coords[0]) / 2).min(2),
                ((self.start.coords[1] + self.end.coords[1]) / 2).min(2),
                ((self.start.coords[2] + self.end.coords[2]) / 2).min(2),
            ],
        }
    }
}

/// Compute the Voronoi region for a set of seed points using Manhattan distance.
/// Returns a mapping from each ternary 3D point to the index of its nearest seed.
/// Ties broken by smallest index.
pub fn voronoi_manhattan_3d(seeds: &[TernaryPoint]) -> Vec<(TernaryPoint, usize)> {
    let mut result = Vec::new();
    for pt in TernaryPoint::all_3d() {
        let mut best_idx = 0;
        let mut best_dist = manhattan_distance(&pt, &seeds[0]);
        for (i, seed) in seeds.iter().enumerate().skip(1) {
            let d = manhattan_distance(&pt, seed);
            if d < best_dist {
                best_dist = d;
                best_idx = i;
            }
        }
        result.push((pt, best_idx));
    }
    result
}

/// Compute the Voronoi region for a set of seed points using Lee distance.
pub fn voronoi_lee_3d(seeds: &[TernaryPoint]) -> Vec<(TernaryPoint, usize)> {
    let mut result = Vec::new();
    for pt in TernaryPoint::all_3d() {
        let mut best_idx = 0;
        let mut best_dist = lee_distance(&pt, &seeds[0]);
        for (i, seed) in seeds.iter().enumerate().skip(1) {
            let d = lee_distance(&pt, seed);
            if d < best_dist {
                best_dist = d;
                best_idx = i;
            }
        }
        result.push((pt, best_idx));
    }
    result
}

/// 2D Voronoi with Manhattan distance.
pub fn voronoi_manhattan_2d(seeds: &[TernaryPoint]) -> Vec<(TernaryPoint, usize)> {
    let mut result = Vec::new();
    for pt in TernaryPoint::all_2d() {
        let mut best_idx = 0;
        let mut best_dist = manhattan_distance(&pt, &seeds[0]);
        for (i, seed) in seeds.iter().enumerate().skip(1) {
            let d = manhattan_distance(&pt, seed);
            if d < best_dist {
                best_dist = d;
                best_idx = i;
            }
        }
        result.push((pt, best_idx));
    }
    result
}

/// Convex hull of ternary points (2D) using the gift wrapping approach.
/// Returns indices into the input slice in hull order.
pub fn convex_hull_2d(points: &[TernaryPoint]) -> Vec<usize> {
    if points.is_empty() {
        return vec![];
    }
    if points.len() == 1 {
        return vec![0];
    }
    if points.len() == 2 {
        return vec![0, 1];
    }

    // Find leftmost-bottommost point
    let mut start = 0;
    for (i, pt) in points.iter().enumerate() {
        if pt.x() < points[start].x() || (pt.x() == points[start].x() && pt.y() < points[start].y()) {
            start = i;
        }
    }

    let mut hull = vec![start];
    let mut current = start;
    loop {
        let mut next = 0;
        for (i, _) in points.iter().enumerate() {
            if i == current {
                continue;
            }
            if next == current {
                next = i;
                continue;
            }
            let cross = cross_product_2d(&points[current], &points[next], &points[i]);
            if cross > 0 {
                next = i;
            } else if cross == 0 && manhattan_distance(&points[current], &points[i])
                > manhattan_distance(&points[current], &points[next])
            {
                next = i;
            }
        }
        if next == start {
            break;
        }
        hull.push(next);
        current = next;
        if hull.len() > points.len() {
            break;
        }
    }
    hull
}

/// 2D cross product of vectors OA and OB (positive if counter-clockwise).
fn cross_product_2d(o: &TernaryPoint, a: &TernaryPoint, b: &TernaryPoint) -> i32 {
    (a.x() as i32 - o.x() as i32) * (b.y() as i32 - o.y() as i32)
        - (a.y() as i32 - o.y() as i32) * (b.x() as i32 - o.x() as i32)
}

/// Compute the area of a polygon defined by ordered ternary 2D points (shoelace formula).
pub fn polygon_area_2d(points: &[TernaryPoint]) -> f64 {
    if points.len() < 3 {
        return 0.0;
    }
    let n = points.len();
    let mut area: i32 = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].x() as i32 * points[j].y() as i32;
        area -= points[j].x() as i32 * points[i].y() as i32;
    }
    (area.abs() as f64) / 2.0
}

/// Compute the volume of the ternary cube (always 27 for the full 3x3x3 grid).
pub fn ternary_grid_volume() -> u32 {
    27
}

/// Compute the area of the full ternary 2D grid (always 9).
pub fn ternary_grid_area() -> u32 {
    9
}

/// Bounding box of a set of ternary points (2D).
/// Returns ((min_x, min_y), (max_x, max_y)).
pub fn bounding_box_2d(points: &[TernaryPoint]) -> ((u8, u8), (u8, u8)) {
    let mut min_x = 2u8; let mut min_y = 2u8;
    let mut max_x = 0u8; let mut max_y = 0u8;
    for pt in points {
        min_x = min_x.min(pt.x());
        min_y = min_y.min(pt.y());
        max_x = max_x.max(pt.x());
        max_y = max_y.max(pt.y());
    }
    ((min_x, min_y), (max_x, max_y))
}

/// Count points inside a bounding box (inclusive).
pub fn points_in_bbox(min: (u8, u8), max: (u8, u8)) -> Vec<TernaryPoint> {
    let mut pts = Vec::new();
    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            if x <= 2 && y <= 2 {
                pts.push(TernaryPoint::new_2d(x, y));
            }
        }
    }
    pts
}

/// Centroid of a set of ternary 2D points.
pub fn centroid_2d(points: &[TernaryPoint]) -> (f64, f64) {
    if points.is_empty() {
        return (0.0, 0.0);
    }
    let sum_x: u32 = points.iter().map(|p| p.x() as u32).sum();
    let sum_y: u32 = points.iter().map(|p| p.y() as u32).sum();
    let n = points.len() as f64;
    (sum_x as f64 / n, sum_y as f64 / n)
}

/// Check if a point is on a line segment (2D).
pub fn point_on_segment(pt: &TernaryPoint, line: &TernaryLine) -> bool {
    let cross = cross_product_2d(&line.start, &line.end, pt);
    if cross != 0 {
        return false;
    }
    let min_x = line.start.x().min(line.end.x());
    let max_x = line.start.x().max(line.end.x());
    let min_y = line.start.y().min(line.end.y());
    let max_y = line.start.y().max(line.end.y());
    pt.x() >= min_x && pt.x() <= max_x && pt.y() >= min_y && pt.y() <= max_y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = TernaryPoint::new(0, 1, 2);
        assert_eq!(p.x(), 0);
        assert_eq!(p.y(), 1);
        assert_eq!(p.z(), 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_point() {
        TernaryPoint::new(3, 0, 0);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = TernaryPoint::new_2d(0, 0);
        let b = TernaryPoint::new_2d(2, 2);
        assert_eq!(manhattan_distance(&a, &b), 4);
    }

    #[test]
    fn test_manhattan_same_point() {
        let a = TernaryPoint::new(1, 1, 1);
        assert_eq!(manhattan_distance(&a, &a), 0);
    }

    #[test]
    fn test_lee_distance() {
        // On Z/3Z, distance between 0 and 2 is min(2, 1) = 1
        let a = TernaryPoint::new_2d(0, 0);
        let b = TernaryPoint::new_2d(2, 0);
        assert_eq!(lee_distance(&a, &b), 1);
    }

    #[test]
    fn test_lee_distance_wrapping() {
        let a = TernaryPoint::new(0, 0, 0);
        let b = TernaryPoint::new(2, 2, 2);
        assert_eq!(lee_distance(&a, &b), 3); // 1+1+1
    }

    #[test]
    fn test_hamming_distance() {
        let a = TernaryPoint::new(0, 1, 2);
        let b = TernaryPoint::new(0, 2, 2);
        assert_eq!(hamming_distance(&a, &b), 1);
    }

    #[test]
    fn test_hamming_all_different() {
        let a = TernaryPoint::new(0, 0, 0);
        let b = TernaryPoint::new(1, 2, 1);
        assert_eq!(hamming_distance(&a, &b), 3);
    }

    #[test]
    fn test_line_midpoint() {
        let line = TernaryLine::new(
            TernaryPoint::new_2d(0, 0),
            TernaryPoint::new_2d(2, 2),
        );
        let mid = line.midpoint();
        assert_eq!(mid.x(), 1);
        assert_eq!(mid.y(), 1);
    }

    #[test]
    fn test_voronoi_2d_single_seed() {
        let seeds = vec![TernaryPoint::new_2d(1, 1)];
        let result = voronoi_manhattan_2d(&seeds);
        assert_eq!(result.len(), 9);
        for (_, idx) in &result {
            assert_eq!(*idx, 0);
        }
    }

    #[test]
    fn test_voronoi_3d_two_seeds() {
        let seeds = vec![
            TernaryPoint::new(0, 0, 0),
            TernaryPoint::new(2, 2, 2),
        ];
        let result = voronoi_manhattan_3d(&seeds);
        assert_eq!(result.len(), 27);
        let count0 = result.iter().filter(|(_, i)| *i == 0).count();
        let count1 = result.iter().filter(|(_, i)| *i == 1).count();
        assert_eq!(count0 + count1, 27);
    }

    #[test]
    fn test_convex_hull_square() {
        let pts = vec![
            TernaryPoint::new_2d(0, 0),
            TernaryPoint::new_2d(2, 0),
            TernaryPoint::new_2d(2, 2),
            TernaryPoint::new_2d(0, 2),
        ];
        let hull = convex_hull_2d(&pts);
        assert!(hull.len() >= 3);
    }

    #[test]
    fn test_convex_hull_with_interior() {
        let pts = vec![
            TernaryPoint::new_2d(0, 0),
            TernaryPoint::new_2d(2, 0),
            TernaryPoint::new_2d(2, 2),
            TernaryPoint::new_2d(0, 2),
            TernaryPoint::new_2d(1, 1), // interior point
        ];
        let hull = convex_hull_2d(&pts);
        // Interior point should not be in hull
        assert!(!hull.contains(&4));
    }

    #[test]
    fn test_polygon_area_square() {
        let pts = vec![
            TernaryPoint::new_2d(0, 0),
            TernaryPoint::new_2d(2, 0),
            TernaryPoint::new_2d(2, 2),
            TernaryPoint::new_2d(0, 2),
        ];
        let area = polygon_area_2d(&pts);
        assert_eq!(area, 4.0);
    }

    #[test]
    fn test_polygon_area_triangle() {
        let pts = vec![
            TernaryPoint::new_2d(0, 0),
            TernaryPoint::new_2d(2, 0),
            TernaryPoint::new_2d(1, 2),
        ];
        let area = polygon_area_2d(&pts);
        assert_eq!(area, 2.0);
    }

    #[test]
    fn test_grid_volume_and_area() {
        assert_eq!(ternary_grid_volume(), 27);
        assert_eq!(ternary_grid_area(), 9);
    }

    #[test]
    fn test_bounding_box() {
        let pts = vec![
            TernaryPoint::new_2d(0, 2),
            TernaryPoint::new_2d(2, 0),
            TernaryPoint::new_2d(1, 1),
        ];
        let ((min_x, min_y), (max_x, max_y)) = bounding_box_2d(&pts);
        assert_eq!((min_x, min_y), (0, 0));
        assert_eq!((max_x, max_y), (2, 2));
    }

    #[test]
    fn test_points_in_bbox() {
        let pts = points_in_bbox((0, 0), (1, 1));
        assert_eq!(pts.len(), 4);
    }

    #[test]
    fn test_centroid() {
        let pts = vec![
            TernaryPoint::new_2d(0, 0),
            TernaryPoint::new_2d(2, 2),
        ];
        let (cx, cy) = centroid_2d(&pts);
        assert_eq!(cx, 1.0);
        assert_eq!(cy, 1.0);
    }

    #[test]
    fn test_point_on_segment() {
        let line = TernaryLine::new(
            TernaryPoint::new_2d(0, 0),
            TernaryPoint::new_2d(2, 0),
        );
        assert!(point_on_segment(&TernaryPoint::new_2d(1, 0), &line));
        assert!(!point_on_segment(&TernaryPoint::new_2d(1, 1), &line));
    }

    #[test]
    fn test_all_2d_count() {
        assert_eq!(TernaryPoint::all_2d().len(), 9);
    }

    #[test]
    fn test_all_3d_count() {
        assert_eq!(TernaryPoint::all_3d().len(), 27);
    }
}
