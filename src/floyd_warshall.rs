//! Floyd-Warshall algorithm.

use crate::result::FloydWarshallResult;
use safe_graph::{Graph, NodeTrait};
use num_traits::Num;
use std::clone::Clone;
use std::cmp::Ord;
use std::cmp::Ordering::{Greater, Less};
use std::cmp::PartialOrd;

/// A trait group for `FloydWarshall`'s weighted edges.
pub trait FloydWarshallTrait: Copy + Num + PartialOrd {}

/// Implement the `FloydWarshallTrait` for all types satisfying bounds.
impl<F> FloydWarshallTrait for F where F: Copy + Num + PartialOrd {}

/// Floyd-Warshall algorithm structure.
///
/// # `FloydWarshall` algorithm is parameterized over:
///
/// - Number type `F` giving a weights to edges.
pub struct FloydWarshall<F: FloydWarshallTrait> {
    /// Operator to be used for weighted edges.
    op: Box<Fn(F, F) -> F>,
    /// Comparison to be used for weighted paths to determine if a newly tested path through `k`
    /// should be added or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::Ordering::{Greater, Less};
    /// use std::cmp::PartialOrd;
    ///
    /// let cmp: Box<Fn(f32, f32) -> bool> = Box::new(|x: f32, y: f32| x.partial_cmp(&y)
    ///   .unwrap_or(Greater) == Less);
    ///
    /// let new_weight = 4.2;
    /// let old_weight = 10.1;
    ///
    /// if cmp(new_weight, old_weight) {
    ///   // Use the new weight and path through `k`.
    /// }
    /// ```
    cmp: Box<Fn(F, F) -> bool>,
    /// Discard loops (e.g. edges starting and ending in the same node) from calculation.
    discard_loops: bool,
}

impl<F: FloydWarshallTrait> FloydWarshall<F> {
    /// Create a new instance of FloydWarshall structure with default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use floyd_warshall_alg::FloydWarshall;
    ///
    /// let alg: FloydWarshall<f32> = FloydWarshall::new();
    /// ```
    pub fn new() -> Self {
        // Initialize defaults.
        let add = Box::new(|x: F, y: F| x + y);
        let sharp_less = Box::new(|x: F, y: F| x.partial_cmp(&y).unwrap_or(Greater) == Less);

        Self {
            op: add,
            cmp: sharp_less,
            discard_loops: true,
        }
    }

    /// Create a new instance of FloydWarshall structure with customized settings.
    ///
    /// You can set:
    /// - the `op` (operator) to be used for weighted edges
    /// - the `cmp` (comparison) to be used for weighted paths
    ///
    /// # Examples
    ///
    /// ```
    /// use floyd_warshall_alg::FloydWarshall;
    /// use std::cmp::Ordering::{Greater, Less};
    ///
    /// let mul = Box::new(|x: f32, y: f32| x * y);
    /// let sharp_greater = Box::new(|x: f32, y: f32| x.partial_cmp(&y).unwrap_or(Less) == Greater);
    ///
    /// let alg: FloydWarshall<f32> = FloydWarshall::new_customized(mul, sharp_greater);
    /// ```
    pub fn new_customized(op: Box<Fn(F, F) -> F>, cmp: Box<Fn(F, F) -> bool>) -> Self {
        Self::new_fully_customized(op, cmp, true)
    }

    /// Create a new instance of FloydWarshall structure with customized settings.
    ///
    /// You can set:
    /// - the `op` (operator) to be used for weighted edges
    /// - the `cmp` (comparison) to be used for weighted paths
    /// - the `discard_loops` to discard loops (e.g. edges starting and ending in the same node)
    ///   from calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use floyd_warshall_alg::FloydWarshall;
    /// use std::cmp::Ordering::{Greater, Less};
    ///
    /// let mul = Box::new(|x: f32, y: f32| x * y);
    /// let sharp_greater = Box::new(|x: f32, y: f32| x.partial_cmp(&y).unwrap_or(Less) == Greater);
    /// let discard_loops = false;
    ///
    /// let alg: FloydWarshall<f32> = FloydWarshall::new_customized(mul, sharp_greater);
    pub fn new_fully_customized(
        op: Box<Fn(F, F) -> F>,
        cmp: Box<Fn(F, F) -> bool>,
        discard_loops: bool,
    ) -> Self {
        Self {
            op,
            cmp,
            discard_loops,
        }
    }

    /// Find all the shortest paths (or best rated paths based on algorithm customized settings).
    ///
    /// The result of type FloydWarshallResult holds both:
    /// - best rates for all possible paths
    /// - next node on the best rated (shortest) path for each possible path
    ///
    /// # Method is parameterized over:
    ///
    /// - Node index / label `N`.
    /// - Number type `E` giving a weight to edges.
    pub fn find_paths<N>(&self, graph: &Graph<N, F>) -> FloydWarshallResult<N, F>
    where
        N: NodeTrait,
    {
        let mut path: Graph<N, F> = graph.clone();
        let mut next: Graph<N, N> = Graph::with_capacity(graph.node_count(), graph.edge_count());

        // Initialize next steps of each edge existing in `graph` with its end node.
        for (a, b, _) in graph.all_edges() {
            next.add_edge(a, b, b);
        }

        // `k` is the "intermediate" node which is currently considered.
        for k in graph.nodes() {
            // `i` is a starting node of the path we try to optimize.
            for i in graph.nodes() {
                // `j` is an end node of the path we try to optimize.
                for j in graph.nodes() {
                    // Skip calculation for loops if requested.
                    if self.discard_loops && !Self::unique(vec![k, i, j]) {
                        continue;
                    }

                    // Calculation of a new weight of the path from `i` to `j` through `k`.
                    let left_operand = path.edge_weight(i, k);
                    let right_operand = path.edge_weight(k, j);

                    // There's nothing to calculate if the left `(i, k)` or right `(k, j)` path misses.
                    if left_operand.is_none() || right_operand.is_none() {
                        continue;
                    }

                    // It is safe to unwrap the operands now.
                    let left_operand = left_operand.unwrap();
                    let right_operand = right_operand.unwrap();

                    // Use the weight operator. It can be customized.
                    let new_weight = (self.op)(*left_operand, *right_operand);

                    // The `(i, j)` path already exists.
                    if let Some(&old_weight) = path.edge_weight(i, j) {
                        // Use the comparison. It can be customized.
                        if (self.cmp)(new_weight, old_weight) {
                            path.add_edge(i, j, new_weight);

                            // The algorithm invariants guarantee the edge exists.
                            let direction = next.edge_weight(i, k).unwrap();
                            next.add_edge(i, j, *direction);
                        }
                    } else {
                        // The path was missing so add a new one.
                        path.add_edge(i, j, new_weight);

                        // The algorithm invariants guarantee the edge exists.
                        let direction = next.edge_weight(i, k).unwrap();
                        next.add_edge(i, j, *direction);
                    }
                }
            }
        }

        FloydWarshallResult::new(path, next)
    }

    /// Are elements unique (no duplicates present).
    fn unique<T: Ord>(mut elements: Vec<T>) -> bool {
        let length = elements.len();

        Self::dedup(&mut elements);

        elements.len() == length
    }

    /// De-duplicate vector elements.
    fn dedup<T: Ord>(elements: &mut Vec<T>) {
        // `sort_unstable` may not preserve the order of equal elements, but it is faster and less
        // memory consuming algorithm.
        elements.sort();
        elements.dedup();
    }
}

#[cfg(test)]
mod tests {
    use crate::floyd_warshall::FloydWarshall;
    use safe_graph::Graph;
    use std::cmp::Ordering::{Greater, Less};

    #[test]
    fn new() {
        let _alg: FloydWarshall<f32> = FloydWarshall::new();
    }

    #[test]
    fn new_customized() {
        let mul = Box::new(|x: f32, y: f32| x * y);
        let sharp_greater = Box::new(|x: f32, y: f32| x.partial_cmp(&y).unwrap_or(Less) == Greater);

        let _alg: FloydWarshall<f32> = FloydWarshall::new_customized(mul, sharp_greater);
    }

    #[test]
    fn new_fully_customized() {
        let mul = Box::new(|x: f32, y: f32| x * y);
        let sharp_less = Box::new(|x: f32, y: f32| x.partial_cmp(&y).unwrap_or(Greater) == Less);
        let discard_loops = false;

        let _alg: FloydWarshall<f32> =
            FloydWarshall::new_fully_customized(mul, sharp_less, discard_loops);
    }

    #[test]
    fn find_paths() {
        let alg: FloydWarshall<f32> = FloydWarshall::new();

        let w_a_b = 0.12;
        let w_a_c = 1.99;
        let w_b_c = 3.0;
        let w_a_d = 2.1;
        let w_a_e = 0.9;
        let w_a_f = 4.44;
        let w_a_g = 0.8;
        let w_g_f = 0.6;
        let w_a_h = 8.8;
        let w_f_h = 1.0;

        let graph = Graph::<_, _>::from_edges(&[
            ("a", "b", w_a_b),
            ("a", "c", w_a_c),
            ("b", "c", w_b_c),
            ("a", "d", w_a_d),
            ("a", "e", w_a_e),
            ("a", "f", w_a_f),
            ("a", "g", w_a_g),
            ("g", "f", w_g_f),
            ("a", "h", w_a_h),
            ("f", "h", w_f_h),
        ]);

        let result = alg.find_paths(&graph);

        let path = result.path;
        let next = result.next;

        // Test that the initial `(a, b)` edge is still the shortest path.
        assert_eq!(path.edge_weight("a", "b"), Some(&w_a_b));
        assert_eq!(next.edge_weight("a", "b"), Some(&"b"));

        // Test that the initial `(a, c)` edge is still the shortest path.
        assert_eq!(path.edge_weight("a", "c"), Some(&w_a_c));
        assert_eq!(next.edge_weight("a", "c"), Some(&"c"));

        // Test that a shorter path was found for the `(a, f)` and it is through `g`.
        assert_eq!(path.edge_weight("a", "f"), Some(&(w_a_g + w_g_f)));
        assert_eq!(next.edge_weight("a", "f"), Some(&"g"));
        assert_eq!(next.edge_weight("g", "f"), Some(&"f"));

        // Test that a shorter path was found for the `(a, f)` and it is through `g` and `f`.
        assert_eq!(path.edge_weight("a", "h"), Some(&(w_a_g + w_g_f + w_f_h)));
        assert_eq!(next.edge_weight("a", "h"), Some(&"g"));
        assert_eq!(next.edge_weight("g", "h"), Some(&"f"));
        assert_eq!(next.edge_weight("f", "h"), Some(&"h"));
    }
}
