use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use libsib::container::{self, Chunk, ChunkKind};
use libsib::grammar::{self, Entry};
use libsib::tree::hex;
use libsib::{Document, check};

const USAGE: &str = "usage:
  sibtool tree <file.sib>...            print the chunk tree
  sibtool unpack <file.sib> <dir>       write each decompressed payload to <dir>
  sibtool roundtrip <file.sib>...       check that parse + serialize reproduces each file
  sibtool parse [--trace] <file.sib>... parse the score payload and report how far the grammar gets
  sibtool check <file.sib>...           decode every chunk (nested files too) and verify the round trip
  sibtool padding [--dump <path>] <file.sib>...  sum the unread record bytes by record path, or print those under <path>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.split_first() {
        Some((cmd, rest)) if cmd == "tree" => rest.iter().try_for_each(|f| tree(Path::new(f))),
        Some((cmd, [file, dir])) if cmd == "unpack" => unpack(Path::new(file), Path::new(dir)),
        Some((cmd, rest)) if cmd == "roundtrip" => roundtrip(rest),
        Some((cmd, rest)) if cmd == "parse" => parse(rest),
        Some((cmd, rest)) if cmd == "check" => check_files(rest),
        Some((cmd, rest)) if cmd == "padding" => padding(rest),
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

type CliResult = Result<(), Box<dyn std::error::Error>>;

fn read(path: &Path) -> Result<Chunk, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(path)?;
    Ok(container::parse(&bytes)?)
}

fn kind_label(kind: ChunkKind) -> String {
    kind.name().map_or_else(|| format!("kind-{}", kind.0), str::to_owned)
}

fn path_label(path: &[ChunkKind]) -> String {
    match path {
        [] => "root".to_owned(),
        _ => path.iter().map(|&k| kind_label(k)).collect::<Vec<_>>().join("/"),
    }
}

fn tree(path: &Path) -> CliResult {
    let root = read(path)?;
    println!("{}", path.display());
    root.walk().iter().for_each(|(kinds, chunk)| {
        let indent = "  ".repeat(kinds.len() + 1);
        let payload = chunk.payload.as_ref().map_or_else(String::new, |p| {
            let seed = p.key.map_or_else(|| "plain".to_owned(), |k| format!("seed {:08x}", k.seed()));
            format!(" payload {} bytes ({seed})", p.data.len())
        });
        println!(
            "{indent}{} version {:08x} flags {:#x}{payload}",
            path_label(kinds),
            chunk.version,
            chunk.other_flags
        );
    });
    Ok(())
}

fn unpack(path: &Path, dir: &Path) -> CliResult {
    std::fs::create_dir_all(dir)?;
    let root = read(path)?;
    root.walk()
        .iter()
        .enumerate()
        .filter_map(|(i, (kinds, chunk))| chunk.payload.as_ref().map(|p| (i, kinds, p)))
        .try_for_each(|(i, kinds, payload)| {
            let name: PathBuf = dir.join(format!("{i:02}-{}.bin", path_label(kinds).replace('/', "_")));
            std::fs::write(&name, &payload.data)?;
            println!("{} ({} bytes)", name.display(), payload.data.len());
            Ok(())
        })
}

fn roundtrip(files: &[String]) -> CliResult {
    let failures = files
        .iter()
        .filter(|f| match roundtrip_one(Path::new(f)) {
            Ok(None) => {
                println!("ok    {f}");
                false
            }
            Ok(Some(at)) => {
                println!("DIFF  {f} (first difference at {at:#x})");
                true
            }
            Err(e) => {
                println!("ERROR {f}: {e}");
                true
            }
        })
        .count();
    match failures {
        0 => Ok(()),
        n => Err(format!("{n} of {} files did not round-trip", files.len()).into()),
    }
}

/// `None` when the rewritten file is byte-identical, otherwise the first differing offset.
fn roundtrip_one(path: &Path) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    let original = std::fs::read(path)?;
    let rewritten = container::serialize(&container::parse(&original)?)?;
    let first_diff = original
        .iter()
        .zip(&rewritten)
        .position(|(a, b)| a != b)
        .or_else(|| (original.len() != rewritten.len()).then(|| original.len().min(rewritten.len())));
    Ok(first_diff)
}

fn parse(args: &[String]) -> CliResult {
    let trace = args.iter().any(|a| a == "--trace");
    let files: Vec<&String> = args.iter().filter(|a| *a != "--trace").collect();
    files.iter().try_for_each(|f| {
        let doc = Document::open(f)?;
        let score = doc.root.child(ChunkKind::SCORE).unwrap_or(&doc.root);
        let data = &score.payload.as_ref().ok_or("file has no score payload")?.data;
        let parsed = grammar::parse(data, Entry::Score, score.version, score.other_flags, trace);
        println!("{f}: {}", parsed.stopped.as_deref().unwrap_or("complete"));
        Ok(())
    })
}

fn check_files(files: &[String]) -> CliResult {
    let failures = files
        .iter()
        .map(|f| -> Result<usize, Box<dyn std::error::Error>> {
            let findings = check::check(&Document::open(f)?)?;
            let bad = findings.iter().filter(|x| x.problem.is_some()).count();
            let bytes: usize = findings.iter().map(|x| x.bytes).sum();
            let skipped: usize = findings.iter().map(|x| x.skipped).sum();
            println!(
                "{:<5} {:>3} chunks {:>9} bytes {:>6.2}% skipped  {f}",
                if bad == 0 { "ok" } else { "FAIL" },
                findings.len(),
                bytes,
                100.0 * skipped as f64 / bytes.max(1) as f64
            );
            findings.iter().filter(|x| x.problem.is_some()).for_each(|x| {
                println!("      {} v{:08x}: {}", path_label(&x.path), x.version, x.problem.as_deref().unwrap_or(""));
            });
            Ok(bad)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|&bad| bad > 0)
        .count();
    match failures {
        0 => Ok(()),
        n => Err(format!("{n} of {} files have chunks that do not decode or round-trip", files.len()).into()),
    }
}

/// Unread record bytes of one record path, summed over files.
#[derive(Default)]
struct PaddingStats {
    bytes: usize,
    records: usize,
    versions: BTreeSet<u32>,
    example: Vec<u8>,
}

impl PaddingStats {
    fn add(&mut self, version: u32, padding: &[u8]) {
        self.bytes += padding.len();
        self.records += 1;
        self.versions.insert(version);
        if self.example.is_empty() {
            self.example = padding[..padding.len().min(24)].to_vec();
        }
    }
}

fn padding(args: &[String]) -> CliResult {
    let (dump, files) = match args {
        [flag, pattern, files @ ..] if flag == "--dump" => (Some(pattern.as_str()), files),
        files => (None, files),
    };
    let mut stats: BTreeMap<String, PaddingStats> = BTreeMap::new();
    files.iter().try_for_each(|f| -> CliResult {
        let doc = Document::open(f)?;
        let version = doc.root.child(ChunkKind::SCORE).unwrap_or(&doc.root).version;
        doc.score()?.root.paddings().into_iter().for_each(|(path, bytes)| {
            if dump.is_some_and(|p| path.contains(p)) {
                println!("{path} [{}]: {}", bytes.len(), hex(bytes));
            }
            stats.entry(path).or_default().add(version, bytes);
        });
        Ok(())
    })?;
    if dump.is_some() {
        return Ok(());
    }
    let mut rows: Vec<_> = stats.into_iter().collect();
    rows.sort_by_key(|(_, s)| std::cmp::Reverse(s.bytes));
    rows.iter().for_each(|(path, s)| {
        let first = s.versions.first().copied().unwrap_or(0);
        let last = s.versions.last().copied().unwrap_or(0);
        println!("{:>10} bytes {:>8} records  v{first:08x}..v{last:08x}  {path}\n      e.g. {}", s.bytes, s.records, hex(&s.example));
    });
    Ok(())
}
