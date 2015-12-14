"""
--- Day 12: JSAbacusFramework.io ---

Santa's Accounting-Elves need help balancing the books after a recent order.
Unfortunately, their accounting software uses a peculiar storage format. That's
where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]),
objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find
all of the numbers throughout the document and add them together.

For example:

[1,2,3] and {"a":2,"b":4} both have a sum of 6.
[[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
{"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
[] and {} both have a sum of 0.
You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?
"""

import os, json
import sys

def json_numbers(data):
    if isinstance(data, int):
        yield data
    elif isinstance(data, list):
        for i in data:
            for n in json_numbers(i):
                yield n
    elif isinstance(data, dict):
        for k, v in data.items():
            for n in json_numbers(k):
                yield n
            for n in json_numbers(v):
                yield n

def json_numbers_unred(data):
    if isinstance(data, int):
        yield data
    elif isinstance(data, list):
        for i in data:
            for n in json_numbers_unred(i):
                yield n
    elif isinstance(data, dict):
        if not "red" in data.values():
            for k, v in data.items():
                for n in json_numbers_unred(k):
                    yield n
                for n in json_numbers_unred(v):
                    yield n

if __name__ == '__main__':
    fn = input("Filename of json to read: ")
    if not (os.path.exists(fn) and os.path.isfile(fn)):
        print("Couldn't find the file: ", fn)
        sys.exit(1)

    with open(fn, 'r') as fp:
        data = json.load(fp)

    print(sum(json_numbers(data)))
    print(sum(json_numbers_unred(data)))
