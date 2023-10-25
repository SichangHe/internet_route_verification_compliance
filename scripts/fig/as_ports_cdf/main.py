"""Run at `scripts/` with `python3 -m fig.as_ports_cdf.main`.
Data are from here:
<https://github.com/SichangHe/internet_route_verification/issues/60#issuecomment-1751551274>
"""
from fig import download_if_missing
from fig.as_ports_cdf import plot


def main():
    download_if_missing(
        "https://github.com/SichangHe/internet_route_verification/files/12908403/as_neighbors_vs_rules.csv",
        "as_neighbors_vs_rules.csv",
    )
    fig, _ = plot()

    pdf_name = "CDF-AS-rules.pdf"
    fig.savefig(pdf_name, bbox_inches="tight")
    fig.set_size_inches(8, 6)
    fig.savefig(pdf_name.replace(".pdf", "-squared.pdf"), bbox_inches="tight")


if __name__ == "__main__":
    main()
