import matplotlib.pyplot as plt
import pandas as pd
from scipy.stats import fit, zipf
from scripts.csv_files import as_set_sizes

FILE = as_set_sizes


def main():
    FILE.download_if_missing()
    df = pd.read_csv(FILE.path)
    df_wo_hash = df[~df['as_set'].str.contains('#')]

    res = fit(zipf, df_wo_hash["size"], [(1.0, 10.0)])
    print(res)
    (alpha, loc) = res.params

    n_bin = 1000
    max_size = max(df_wo_hash["size"])

    x = range(1, max_size + 1)
    fitted_data = zipf.pmf(x, alpha, loc=loc)

    # Plotting the fitted distribution against the empirical data
    plt.bar(x, fitted_data, alpha=0.5, color="yellow", label="Fitted Zipf Distribution")
    plt.hist(
        df_wo_hash["size"],
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
