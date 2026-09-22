# Paging

`paging` is a `no_std` framework for architecture-independent page-table walks and updates.

An architecture implementation supplies its granules, descriptor formats, descriptor layouts, regimes, semantic attribute codecs, table access, frame allocation, and live invalidation behavior.

See [aarch64-vmsa](https://github.com/joe3925/aarch64-vmsa) for an example implementation.
