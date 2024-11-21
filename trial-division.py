#!/usr/bin/env python
# -*- coding: utf-8 -*-

import math
from tqdm import tqdm

def trial_division(N):
    sqrt_N = math.ceil(math.sqrt(N))
    factors = []
    while N % 2 == 0:
        factors.append(2)
        N //= 2
    for i in tqdm(range(3, sqrt_N, 2)):
        while N % i == 0:
            factors.append(i)
            N //= i
        if N == 1:
            return factors
    factors.append(N)
    return factors

if __name__ == "__main__":
    M13 = (1<<13)-1
    M17 = (1<<17)-1
    N0 = M13 * M17
    N = 237540380304900134239
    print(trial_division(N))


