"""Time one merge of the four operation logs."""

import timeit


def merge_benchmark():
    """Return the seconds one merge takes over a representative log set."""
    return timeit.timeit("merge(logs)", number=1000)
