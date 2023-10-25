import os

import matplotlib.pyplot as plt
import requests

plt.rcParams["axes.xmargin"] = 0
plt.rcParams["axes.ymargin"] = 0
plt.rcParams["pdf.fonttype"] = 42
plt.rcParams["ps.fonttype"] = 42


def download_if_missing(url: str, path: str):
    if os.path.exists(path):
        return
    response = requests.get(url)
    response.raise_for_status()
    with open(path, "wb") as f:
        f.write(response.content)
    print(f"Downloaded {url} -> {path}.")
