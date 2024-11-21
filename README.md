# Trial Division
Code for naive trial division in Rust.
This was written to get particular numbers for [this stack exchange question](https://crypto.stackexchange.com/a/113521/45690), where someone asked how to factor $N = 237540380304900134239$ without external tools.
An initial implementation of trial division in Python ended up taking (estimated) >= 25 minutes, so I decided to try a little harder to see if this could be brought down to something more reasonable.
Using `rayon`, I was able to factor $N$ in ~10.5 seconds on my (not particularly nice) laptop.
