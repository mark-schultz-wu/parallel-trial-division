# Trial Division
Code for naive trial division in Python and Rust.
This was written to get particular numbers for [this stack exchange question](https://crypto.stackexchange.com/a/113521/45690), where someone asked how to factor $N = 237540380304900134239$ without external tools.
As the (naive) implementation in Python is slow, I decided to try to speed it up, by

1. Rewriting it in Rust™, and then
2. parallelizing this Rust implementation with `rayon`.

# Timings

As the entire point of this was to obtain timing estimates for various ways of implementing trial division, I've included those estimates below (for the above $N$) on various machines.
These machines are

* "Laptop": Intel i7 1165G7 (4  cores, 2.8GHz) and 16GB of RAM, and
* "Desktop": Ryzen 9 5900X  (12 cores, 3.70 GHz) with 32GB of RAM.

| Implementation (Laptop) | Time (s) |
| --- | --- |
| Serial Python | ??? |
| Serial Rust | 20.86 |
| Parallel Rust | 10.81 |

| Implementation (Desktop) | Time (s) |
| --- | --- |
| Serial Python | 689 |
| Serial Rust | 16.78 |
| Parallel Rust | 2.53 |



# Usage

I've wrapped both Rust implementations with a small CLI. The syntax is

    cargo run --release [NUMBER]

to factor NUMBER in parallel, and

    cargo run --release [NUMBER] --serial

to factor NUMBER in serial.

I've also included the naive `python3` implementation, but it is exceedingly boring, slow, and not even portable (it requires `tqdm` to be installed, though one could easily remove this dependency).
There is no reason I can think that anyone should want to use it ever --- I mainly include it as a reference for the above measurements.
