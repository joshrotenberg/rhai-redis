# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-01-19

### Added
- Comprehensive module support for specialized Redis commands:
  - **Bloom Filter module**: bloom_add, bloom_exists, bloom_madd, bloom_mexists
  - **HyperLogLog module**: pfadd, pfcount, pfmerge
  - **Geo module**: geoadd, geodist, georadius, geopos, geohash
  - **Bitmap module**: setbit, getbit, bitcount, bitop, bitpos
- Enhanced string operations with microsecond-precision benchmarking support
- Comprehensive rustdoc documentation for all modules
- GitHub Actions CI workflow for automated testing
- Full test coverage for all new modules
- Performance benchmarks for Redis operations

### Changed
- Improved error handling across all modules
- Enhanced documentation with detailed examples
- Updated README with professional formatting (no emojis)

### Fixed
- String operations now properly handle edge cases
- Module organization for better maintainability

## [0.1.0] - 2025-01-18

### Initial Release
- Core Redis operations support through Rhai scripting
- Basic modules: strings, hashes, lists, sets, sorted sets, keys, transactions
- Redis Search support with query building
- JSON operations support
- Stream operations
- Pub/Sub functionality
- Utility functions for testing
- Example scripts demonstrating usage
