"""
Python implementation of day4 solution, for benchmarking purposes.

NOTE: due to the GIL and the hassle of figuring out the `multiprocessing`
module, this code is single-threaded.
"""

import sys
from hashlib import md5

def work_items():
    val = 0
    while val < sys.maxsize:
        yield val
        val += 1

def mine_coin(secret, leading_zeros=5):
    for item in work_items():
        it = str(item)
        lead = md5((secret + it).encode('utf-8')).hexdigest()[:leading_zeros]
        if lead == '0' * leading_zeros:
            return item

if __name__ == '__main__':
    secret = input("Enter your secret key here: ")
    coin = mine_coin(secret)
    if coin is None:
        print("No coin found")
    print("Coin:", coin)
