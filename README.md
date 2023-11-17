# Overview ![Crates.io](https://img.shields.io/crates/v/simple-cpu-benchmark)

Super simple CPU benchmark with no external dependencies. Designed to be super
simple by default (single command without args), and quick (takes at most 30
seconds).

The benchmark calculates score by running a Fibonacci function with increasing
factor until CPU can no longer calculate it under a second. The final result
for a single thread - maximum Fibonacci number it was able to calculate. For
multithreaded test runs a test for each logical thread in parallel and sums up
the results.

# Description

todo

# Usage example

Run simple-cpu-benchmark to see single and multi-threaded results.

# TODO

- Statistics and export
- Options (help, version, export)
- Measure background CPU activity
