#!/usr/bin/env python
# -*- coding: utf-8 -*-

import math
from tqdm import tqdm

def trial_divide(N, i):
    if N % i == 0:
        return (i, True)
    return (0, False)

def trial_division(N):
    sqrt_N = math.ceil(math.sqrt(N))
    factors = []
    while N % 2 != 0:
        factors.append(2)
        N //= 2
    for i in tqdm(range(3, sqrt_N, 2)):
        if N % i == 0:
            factors.append(i)
            N //= i
        if N == 1:
            return factors

if __name__ == "__main__":
    N = 237540380304900134239
    trial_division(N)


