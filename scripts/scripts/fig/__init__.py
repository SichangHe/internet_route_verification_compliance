import matplotlib.pyplot as plt
import pandas as pd

plt.rcParams["axes.xmargin"] = 0
plt.rcParams["axes.ymargin"] = 0
plt.rcParams["pdf.fonttype"] = 42
plt.rcParams["ps.fonttype"] = 42


def smart_sample(same: tuple[pd.Series]):
    """Only sample the indexes of boundary values.
    All same (plural of "series") should have the same length."""
    size = len(same[0])
    max_index = size - 1

    indexes: list[int] = []
    values = tuple([] for _ in same)
    old_value = None
    retaining = False
    for index in range(size):
        value = tuple(series[index] for series in same)
        if value == old_value and index != max_index:
            retaining = True
            continue
        if retaining:
            assert old_value is not None
            indexes.append(index - 1)
            for vs, v in zip(values, old_value):
                vs.append(v)
            retaining = False
        old_value = value
        indexes.append(index)
        for vs, v in zip(values, value):
            vs.append(v)
    return indexes, values
