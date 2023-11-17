"""Run at `scripts/` with `python3 -m scripts.fig.as_stacked_area`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/89>
"""
import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.axes import Axes
from matplotlib.figure import Figure
from scripts.csv_files import as_stats
from scripts.fig import smart_sample

FILE = as_stats
PORTS = ("import", "export")
TAGS = ("ok", "skip", "unrec", "meh", "err")


def plot():
    df = pd.read_csv(FILE.path)

    dfs: dict[str, pd.DataFrame] = {}
    figs: dict[str, Figure] = {}
    axs: dict[str, Axes] = {}
    for port in PORTS:
        d = df[["aut_num"]].copy()
        d["total"] = sum(df[f"{port}_{tag}"] for tag in TAGS)
        for tag in TAGS:
            d[f"%{tag}"] = df[f"{port}_{tag}"] / d["total"] * 100.0
        d.dropna(inplace=True)
        d.sort_values(
            by=[f"%{tag}" for tag in ("ok", "err", "skip", "unrec", "meh")],
            ascending=[False, True, False, False, False],
            ignore_index=True,
            inplace=True,
        )
        dfs[port] = d
    d = df[["aut_num"]].copy()
    d["total"] = sum(df[f"{port}_{tag}"] for tag in TAGS for port in PORTS)
    for tag in TAGS:
        d[f"%{tag}"] = sum(df[f"{port}_{tag}"] for port in PORTS) / d["total"] * 100.0
    d.dropna(inplace=True)
    d.sort_values(
        by=[f"%{tag}" for tag in ("ok", "err", "skip", "unrec", "meh")],
        ascending=[False, True, False, False, False],
        ignore_index=True,
        inplace=True,
    )
    dfs["exchange"] = d
    for key, d in dfs.items():
        indexes, values = smart_sample(tuple(d[f"%{tag}"] for tag in TAGS))

        fig, ax = plt.subplots(figsize=(16, 9))
        figs[key], axs[key] = fig, ax
        fig.tight_layout()
        ax.stackplot(
            indexes,
            values,
            labels=[f"%{tag}" for tag in TAGS],
        )
        ax.set_xlabel("AS", fontsize=16)
        ax.set_ylabel(f"Percentage of {key}", fontsize=16)
        ax.tick_params(axis="both", labelsize=14)
        ax.grid()
        ax.legend(loc="lower left", fontsize=14)

    # For checking.
    # figs["import"].show()

    return figs, axs, dfs


def main():
    FILE.download_if_missing()
    figs, _, _ = plot()

    for key, fig in figs.items():
        pdf_name = f"AS-{key}-percentages-stacked-area.pdf"
        fig.savefig(pdf_name, bbox_inches="tight")
        fig.set_size_inches(8, 6)
        fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
