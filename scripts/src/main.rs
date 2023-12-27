use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use rayon::prelude::*;

struct CsvFile {
    path: &'static str,
    url: &'static str,
}

#[allow(non_snake_case)]
const fn CsvFile(path: &'static str, url: &'static str) -> CsvFile {
    CsvFile { path, url }
}

/// Copied from `scripts/csv_files.py`.
const ROUTE_STATS_ALL: [CsvFile;59]  = [
    CsvFile(
        "pacwave.lax--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/pacwave.lax--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.amsix--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.amsix--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.bdix--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.bdix--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.bknix--route_stats.csv",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.bknix--route_stats.csv",
    ),
    CsvFile(
        "route-views.chicago--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.chicago--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.chile--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.chile--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.decix-jhb--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.decix-jhb--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.eqix--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.eqix--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.flix--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.flix--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.fortaleza--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.fortaleza--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.gixa--route_stats.csv",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.gixa--route_stats.csv",
    ),
    CsvFile(
        "route-views.gorex--route_stats.csv",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.gorex--route_stats.csv",
    ),
    CsvFile(
        "route-views.isc--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.isc--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.kixp--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.kixp--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.linx--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.linx--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.mwix--route_stats.csv",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.mwix--route_stats.csv",
    ),
    CsvFile(
        "route-views.napafrica--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.napafrica--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.nwax--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.nwax--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.ny--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.ny--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.perth--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.perth--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.peru--route_stats.csv",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.peru--route_stats.csv",
    ),
    CsvFile(
        "route-views.phoix--route_stats.csv",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.phoix--route_stats.csv",
    ),
    CsvFile(
        "route-views.rio--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.rio--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.sfmix--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.sfmix--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.sg--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.sg--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.soxrs--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.soxrs--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.sydney--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.sydney--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.telxatl--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.telxatl--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.uaeix--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.uaeix--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views.wide--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views.wide--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views2--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views2--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views2.saopaulo--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views2.saopaulo--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views3--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views3--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views4--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views4--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views5--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views5--route_stats.csv.gz",
    ),
    CsvFile(
        "route-views6--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/route-views6--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc00--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc00--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc01--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc01--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc03--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc03--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc04--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc04--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc05--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc05--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc06--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc06--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc07--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc07--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc10--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc10--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc11--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc11--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc12--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc12--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc13--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc13--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc14--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc14--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc15--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc15--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc16--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc16--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc18--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc18--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc19--route_stats.csv",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc19--route_stats.csv",
    ),
    CsvFile(
        "rrc20--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc20--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc21--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc21--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc22--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc22--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc23--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc23--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc24--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc24--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc25--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc25--route_stats.csv.gz",
    ),
    CsvFile(
        "rrc26--route_stats.csv.gz",
        "https://github.com/SichangHe/internet_route_verification/releases/download/data-116/rrc26--route_stats.csv.gz",
    ),
];

#[derive(Clone, Debug)]
struct Row {
    import_ok: u16,
    export_ok: u16,
    import_skip: u16,
    export_skip: u16,
    import_unrec: u16,
    export_unrec: u16,
    import_meh: u16,
    export_meh: u16,
    import_err: u16,
    export_err: u16,
}

#[allow(clippy::never_loop)]
fn process_file<R: Read>(file: BufReader<R>) -> Result<()> {
    let mut lines = file.lines();

    let _header = lines.next().context("No header")??;
    for maybe_line in lines {
        let line = maybe_line.context("Failed to read line")?;
        let mut fields = line.split(',');

        let mut next_field = || {
            fields
                .next()
                .context("Missing expected field")?
                .parse()
                .context("Failed to parse field")
        };
        let row = Row {
            import_ok: next_field()?,
            export_ok: next_field()?,
            import_skip: next_field()?,
            export_skip: next_field()?,
            import_unrec: next_field()?,
            export_unrec: next_field()?,
            import_meh: next_field()?,
            export_meh: next_field()?,
            import_err: next_field()?,
            export_err: next_field()?,
        };

        println!("{row:?}");
        break;
    }

    Ok(())
}

fn main() {
    ROUTE_STATS_ALL
        .par_iter()
        .map(|CsvFile { path, url: _ }| {
            let file = File::open(path).context("Failed to open file")?;
            if path.ends_with(".gz") {
                let file = BufReader::new(GzDecoder::new(file));
                process_file(file).with_context(|| format!("Failed to process file {}", path))
            } else {
                let file = BufReader::new(file);
                process_file(file).with_context(|| format!("Failed to process file {}", path))
            }
        })
        .collect::<Result<()>>()
        .unwrap();
}
