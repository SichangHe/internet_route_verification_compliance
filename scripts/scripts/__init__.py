import os

import requests


def download_if_missing(url: str, path: str):
    if os.path.exists(path):
        return
    response = requests.get(url)
    response.raise_for_status()
    with open(path, "wb") as f:
        f.write(response.content)
    print(f"Downloaded {url} -> {path}.")
