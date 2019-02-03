# Floyd-Warshall algorithm

Rust implementation of Floyd-Warshall algorithm supporting customization.

|Crate|Documentation|Travis CI|CodeCov|
|:---:|:-----------:|:-------:|:-----:|
|[![Crate](http://meritbadge.herokuapp.com/floyd-warshall-alg)](https://crates.io/crates/floyd-warshall-alg)|[![Documentation](https://docs.rs/floyd-warshall-alg/badge.svg)](https://docs.rs/floyd-warshall-alg)|[![Build Status](https://travis-ci.org/dalibor-matura/floyd-warshall-alg.svg?branch=master)](https://travis-ci.org/dalibor-matura/floyd-warshall-alg)|[![codecov](https://codecov.io/gh/dalibor-matura/floyd-warshall-alg/branch/master/graph/badge.svg)](https://codecov.io/gh/dalibor-matura/floyd-warshall-alg)

## Customization

Operator
* Is used to calculate a new path weight when trying the path through additional node `k`.
* By default set to be standard `addition`.
* Can be used to replace default `addition` to something else (e.g. multiplication).

Comparison
* Is used for a newly tested path (through `k`) and the old path to determine if the new path weight is better.
* By default set to be standard `min` (return `true` if the left argument is lower than the right).
* Can be used to replace default `min` to something else (e.g. `max`).

## Dependencies

Safe Graph ([safe-graph](https://crates.io/crates/safe-graph))
* Rust implementation of Graph (refactored version of GraphMap from petgraph).
* My own crate.

Numeric traits ([num-traits](https://crates.io/crates/num-traits))
* Numeric traits for generic mathematics in Rust.
* Third-party crate with 6+ millions downloads (so even not being in a stable release, the high usage gives it a high testing coverage).


## License
Licensed under the General Public License (GPL), version 3 ([LICENSE](https://github.com/dalibor-matura/floyd-warshall-alg/blob/master/LICENSE) http://www.gnu.org/licenses/gpl-3.0.en.html).
