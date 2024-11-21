#!/usr/bin/env python
# -*- coding: utf-8 -*-

import math
from tqdm import tqdm



import concurrent.futures

def trial_divide(N, i):
    if N % i == 0:
        return (i, True)
    return (0, False)

def parallel_trial_division(N):
    factors = []
    while N % 2 != 0:
        factors.append(2)
        N //= 2
    sqrt_N = math.ceil(math.sqrt(N))
    odd_numbers = range(3, sqrt_N, 2)
    with tqdm(odd_numbers) as possible_divisors:
        with concurrent.futures.ThreadPoolExecutor() as executor:
            futures = executor.map(lambda i: trial_divide(N,i), odd_numbers)
            for possible_factor in concurrent.futures.as_completed(futures):
                possible_divisors.update()
                if possible_factor[1]:
                    factors.append(possible_factor[0])
    return factors

def serial_trial_division(N):
    sqrt_N = math.ceil(math.sqrt(N))
    factors = []
    while N % 2 != 0:
        factors.append(2)
        N //= 2
    for i in tqdm.trange(3, sqrt_N, 2):
        if N % i == 0:
            factors.append(i)
            N //= i
        if N == 1:
            return factors

if __name__ == "__main__":
    N = 237540380304900134239
    # serial_trial_division(N)
    parallel_trial_division(N)
    p = 2**13 - 1
    q = 2**17 - 1
    Nsmall = p*q
    # parallel_trial_division(Nsmall)


