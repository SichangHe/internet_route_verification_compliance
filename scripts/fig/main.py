import gc

from fig import as_pair_cdf, as_rules_cdf, as_stacked_area, route_port_stacked_area


def main():
    mods = [
        as_pair_cdf,
        as_rules_cdf,
        as_stacked_area,
        route_port_stacked_area,
        # route_spec_cdf,
    ]
    for mod in mods:
        print(f"Running {mod.__name__}.")
        mod.main()
        gc.collect()


if __name__ == "__main__":
    main()
