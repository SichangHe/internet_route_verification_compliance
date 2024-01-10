"""Run at `scripts/` with `python3 -m scripts.stats.as_set_size_fitting`."""
import matplotlib.pyplot as plt
import pandas as pd
from scipy.stats import fit, zipf
from scripts.csv_files import as_set_sizes

FILE = as_set_sizes


def main():
    FILE.download_if_missing()
    df_raw = pd.read_csv(FILE.path)
    df_wo_hash = df_raw[~df_raw["as_set"].str.contains("#")]

    print("Fitting AS Set sizes.")
    df = df_wo_hash[df_wo_hash["size"] > 0]
    res = fit(zipf, df["size"], [(1.0, 10.0)])
    print(res)
    (alpha, loc) = res.params

    n_bin = 1000
    max_size = max(df["size"])

    x = range(1, max_size + 1)
    fitted_data = zipf.pmf(x, alpha, loc=loc)

    plt.bar(x, fitted_data, alpha=0.5, color="yellow", label="Fitted Zipf Distribution")
    plt.hist(
        df["size"],
        bins=n_bin,
        density=True,
        alpha=0.5,
        color="blue",
        label="Empirical Data",
    )
    plt.legend()
    plt.xlabel("Value")
    plt.ylabel("Frequency")
    plt.xscale("log")
    plt.yscale("log")
    plt.title("Fitted Zipf Distribution vs Empirical Data")
    plt.show()

    print("Fitting AS Set nesting depths.")
    df = df_wo_hash[df_wo_hash["depth"] > 0]
    res = fit(zipf, df["depth"], [(1.0, 10.0)])
    print(res)
    (alpha, loc) = res.params

    n_bin = 1000
    max_size = max(df["depth"])

    x = range(1, max_size + 1)
    fitted_data = zipf.pmf(x, alpha, loc=loc)

    plt.bar(x, fitted_data, alpha=0.5, color="yellow", label="Fitted Zipf Distribution")
    plt.hist(
        df["depth"],
        bins=n_bin,
        density=True,
        alpha=0.5,
        color="blue",
        label="Empirical Data",
    )
    plt.legend()
    plt.xlabel("Value")
    plt.ylabel("Frequency")
    plt.xscale("log")
    plt.yscale("log")
    plt.title("Fitted Zipf Distribution vs Empirical Data")
    plt.show()


main() if __name__ == "__main__" else None
