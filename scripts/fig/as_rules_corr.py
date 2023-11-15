"""Run at `scripts/` with `python3 -m fig.as_rules_corr`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/95>
"""
import pandas as pd
from fig import download_if_missing

NEIGHBORS = ("provider", "peer", "customer")
RULES = ("import", "export")


def main():
    download_if_missing(
        "https://github.com/SichangHe/internet_route_verification/files/13346943/as_neighbors_vs_rules2.csv",
        "as_neighbors_vs_rules2.csv",
    )
    df_raw = pd.read_csv("as_neighbors_vs_rules2.csv")
    # Remove ASes not in IRR or not in AS Relationship DB.
    df = df_raw.drop(
        df_raw[(df_raw["import"] == -1) | (df_raw["provider"] == -1)].index
    )
    df["neighbors"] = sum(df[neighbor] for neighbor in NEIGHBORS)
    df["rules"] = sum(df[rule] for rule in RULES)

    neighbors = list(NEIGHBORS) + ["neighbors"]
    rules = list(RULES) + ["rules"]
    print("Pearson, Kendall Tau, and Spearman rank correlation.")
    for neighbor in neighbors:
        for rule in rules:
            corrs = ", ".join(
                [
                    f"{df[neighbor].corr(df[rule], method=method):.3f}"
                    for method in ("pearson", "kendall", "spearman")
                ]
            )
            print(f"{corrs}: between {neighbor} and {rule}.")


if __name__ == "__main__":
    main()
