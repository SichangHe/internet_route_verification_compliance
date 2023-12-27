#![allow(clippy::unnecessary_lazy_evaluations)]
use core::cmp::Ordering::*;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use rayon::prelude::*;

#[allow(dead_code)]
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

impl Row {
    fn percentages(
        &self,
    ) -> (
        Option<Percentages>,
        Option<Percentages>,
        Option<Percentages>,
    ) {
        let Self {
            import_ok,
            export_ok,
            import_skip,
            export_skip,
            import_unrec,
            export_unrec,
            import_meh,
            export_meh,
            import_err,
            export_err,
        } = self;
        let (
            import_ok,
            export_ok,
            import_skip,
            export_skip,
            import_unrec,
            export_unrec,
            import_meh,
            export_meh,
            import_err,
            export_err,
        ) = (
            *import_ok as f64,
            *export_ok as f64,
            *import_skip as f64,
            *export_skip as f64,
            *import_unrec as f64,
            *export_unrec as f64,
            *import_meh as f64,
            *export_meh as f64,
            *import_err as f64,
            *export_err as f64,
        );

        let import_total = import_ok + import_skip + import_unrec + import_meh + import_err;
        let export_total = export_ok + export_skip + export_unrec + export_meh + export_err;
        let total = import_total + export_total;

        let import_percentages = (import_total > 0.0).then(|| Percentages {
            ok: import_ok * 100.0 / import_total,
            skip: import_skip * 100.0 / import_total,
            unrec: import_unrec * 100.0 / import_total,
            meh: import_meh * 100.0 / import_total,
            err: import_err * 100.0 / import_total,
        });

        let export_percentages = (export_total > 0.0).then(|| Percentages {
            ok: export_ok * 100.0 / export_total,
            skip: export_skip * 100.0 / export_total,
            unrec: export_unrec * 100.0 / export_total,
            meh: export_meh * 100.0 / export_total,
            err: export_err * 100.0 / export_total,
        });

        let total_percentages = (total > 0.0).then(|| Percentages {
            ok: (import_ok + export_ok) * 100.0 / total,
            skip: (import_skip + export_skip) * 100.0 / total,
            unrec: (import_unrec + export_unrec) * 100.0 / total,
            meh: (import_meh + export_meh) * 100.0 / total,
            err: (import_err + export_err) * 100.0 / total,
        });

        (import_percentages, export_percentages, total_percentages)
    }
}

/// None of the fields should be NaN.
#[derive(Clone, Debug, Default, PartialEq)]
struct Percentages {
    ok: f64,
    skip: f64,
    unrec: f64,
    meh: f64,
    err: f64,
}

impl Eq for Percentages {}

impl Ord for Percentages {
    /// Sort by `ok` descending, then `err` ascending, then `skip` descending,
    /// then `unrec` descending, then `meh` descending.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self
            .ok
            .partial_cmp(&other.ok)
            .expect("`Percentages` should not have NaN")
        {
            Equal => {}
            ord => return ord.reverse(),
        }
        match self
            .err
            .partial_cmp(&other.err)
            .expect("`Percentages` should not have NaN")
        {
            Equal => {}
            ord => return ord,
        }
        match self
            .skip
            .partial_cmp(&other.skip)
            .expect("`Percentages` should not have NaN")
        {
            Equal => {}
            ord => return ord.reverse(),
        }
        match self
            .unrec
            .partial_cmp(&other.unrec)
            .expect("`Percentages` should not have NaN")
        {
            Equal => {}
            ord => return ord.reverse(),
        }
        self.meh
            .partial_cmp(&other.meh)
            .expect("`Percentages` should not have NaN")
            .reverse()
    }
}

impl PartialOrd for Percentages {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn process_file<R: Read>(file: BufReader<R>, stats: Stats) -> Result<()> {
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

        let (import_percentages, export_percentages, total_percentages) = row.percentages();

        for (item, pool) in [import_percentages, export_percentages, total_percentages]
            .into_iter()
            .zip(&stats)
        {
            if let Some(percentages) = item {
                let mut pool = pool.lock().expect("`pool` should not have been poisoned.");
                *pool.entry(percentages).or_default() += 1;
            }
        }
    }

    Ok(())
}

#[derive(Clone)]
struct Stats {
    import: Arc<Mutex<BTreeMap<Percentages, u32>>>,
    export: Arc<Mutex<BTreeMap<Percentages, u32>>>,
    total: Arc<Mutex<BTreeMap<Percentages, u32>>>,
}

impl Stats {
    fn new() -> Self {
        Self {
            import: Arc::new(Mutex::new(BTreeMap::new())),
            export: Arc::new(Mutex::new(BTreeMap::new())),
            total: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
}

impl<'a> IntoIterator for &'a Stats {
    type Item = &'a Arc<Mutex<BTreeMap<Percentages, u32>>>;

    type IntoIter = std::array::IntoIter<&'a Arc<Mutex<BTreeMap<Percentages, u32>>>, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [&self.import, &self.export, &self.total].into_iter()
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let stats = Stats::new();

    ROUTE_STATS_ALL
        .par_iter()
        .map(|CsvFile { path, url: _ }| {
            let file = File::open(path).context("Failed to open file")?;
            if path.ends_with(".gz") {
                let file = BufReader::new(GzDecoder::new(file));
                process_file(file, stats.clone())
                    .with_context(|| format!("Failed to process file {}", path))
            } else {
                let file = BufReader::new(file);
                process_file(file, stats.clone())
                    .with_context(|| format!("Failed to process file {}", path))?;
                println!("Processed `{}`", path);

                Ok(())
            }
        })
        .collect::<Result<()>>()
        .unwrap();

    for (stats, filename) in stats.into_iter().zip([
        "route_all_import_stats.csv",
        "route_all_export_stats.csv",
        "route_all_total_stats.csv",
    ]) {
        let mut file = BufWriter::new(File::create(filename).unwrap());
        file.write_all(b"%ok,%skip,%unrec,%meh,%err,count\n")
            .unwrap();

        let stats = stats
            .lock()
            .expect("`stats` should not have been poisoned.");
        for (
            Percentages {
                ok,
                skip,
                unrec,
                meh,
                err,
            },
            count,
        ) in stats.iter()
        {
            file.write_all(ok.to_string().as_bytes()).unwrap();
            file.write_all(b",").unwrap();
            file.write_all(skip.to_string().as_bytes()).unwrap();
            file.write_all(b",").unwrap();
            file.write_all(unrec.to_string().as_bytes()).unwrap();
            file.write_all(b",").unwrap();
            file.write_all(meh.to_string().as_bytes()).unwrap();
            file.write_all(b",").unwrap();
            file.write_all(err.to_string().as_bytes()).unwrap();
            file.write_all(b",").unwrap();
            file.write_all(count.to_string().as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }

        file.flush().unwrap();
    }
}
