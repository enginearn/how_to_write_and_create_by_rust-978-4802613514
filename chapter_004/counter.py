#!/bin/bash/env python3


class Counter(object):
    value = 0

    def inc(self) -> None:
        self.value += 1
        print(f"value = {self.value}")


def count(counter) -> Counter():
    counter.inc()


a = Counter()
count(a)
count(a)
