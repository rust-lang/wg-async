//! Searches for broken local paths in markdown files and attempts to repair them.
//!
//! Usage: cargo run --bin=fixlinks ./**/*.md

use std::{collections::{BTreeSet, HashMap}, fs, ops::Range, path::Component};
use camino::*;
use pathdiff::diff_paths;
use regex::{Match, Regex};

#[allow(unused)]
struct FilenameEntry {
    file: Utf8PathBuf,
    reported: bool,
}

fn main() {
    let files: Vec<Utf8PathBuf> = std::env::args().skip(1).map(Into::into).collect();
    let mut filenames = HashMap::<String, Vec<Utf8PathBuf>>::new();
    for file in &files {
        let name = file.file_name().unwrap();
        filenames.entry(name.to_owned()).or_default().push(file.clone());
    }

    for (name, paths) in &filenames {
        if paths.len() > 1 {
            eprintln!("Note: Duplicate filename: {}", name);
            for path in paths {
                eprintln!("- {}", path);
            }
        }
    }

    let mut cx = Context::new(filenames);

    for file in &files {
        let contents = fs::read_to_string(&file).unwrap();
        cx.check_file(file, &contents);
    }

    eprintln!("{:#?}", cx.count);
}

struct Context {
    filenames: HashMap::<String, Vec<Utf8PathBuf>>,
    count: Counts,
}

#[derive(Default, Debug)]
struct Counts {
    urls: usize,
    paths: usize,
    missing: usize,
    matches: usize,
    ambiguous: usize,
    ties: usize,
}

impl Context {
    fn new(filenames: HashMap::<String, Vec<Utf8PathBuf>>) -> Self {
        Context {
            filenames,
            count: Default::default(),
        }
    }

    fn check_file<'c>(&mut self, file: &Utf8Path, contents: &'c str) {
        let re_linkref = Regex::new(r"^\[(.*?)\]: (.*)").unwrap();
        let re_inline = Regex::new(r"\[([^\[\]]*)\]\((.*?)\)").unwrap();

        struct UrlMatch {
            line_no: usize,
            source_range: Range<usize>,
        }
        let mut matches = vec![];
        let mut add_match = |line: &'c str, line_no, mat: Match| {
            // Safety: line and contents are from the same allocation.
            let line_start_idx = unsafe { line.as_ptr().offset_from(contents.as_ptr()) } as usize;
            let source_range = (line_start_idx + mat.start())..(line_start_idx + mat.end());
            matches.push(UrlMatch { line_no, source_range });
        };

        // Match against linkrefs, then inline links.
        let mut linkrefs = BTreeSet::new();
        for (idx, line) in contents.lines().enumerate() {
            let line_no = idx + 1;
            for cap in re_linkref.captures_iter(line) {
                add_match(line, line_no, cap.get(2).unwrap());
                linkrefs.insert(cap.get(1).unwrap().as_str());
            }
        }
        for (idx, line) in contents.lines().enumerate() {
            let line_no = idx + 1;
            for cap in re_inline.captures_iter(line) {
                if let Some(url) = cap.get(2) {
                    if !linkrefs.contains(url.as_str()) {
                        add_match(line, line_no, url);
                    }
                }
            }
        }

        // We need to process through matches in order so we can build the
        // modified contents sequentially.
        matches.sort_by_key(|um| um.source_range.start);

        let mut modified = String::new();
        let mut source_bytes_written = 0;
        for UrlMatch { line_no, source_range } in matches {
            if let Some(replacement) = self.check_url(file, line_no, &contents[source_range.clone()]) {
                modified.push_str(&contents[source_bytes_written..source_range.start]);
                modified.push_str(&replacement);
                source_bytes_written = source_range.end;
            }
        };

        if !modified.is_empty() {
            modified.push_str(&contents[source_bytes_written..]);
            fs::write(file, modified).unwrap();
        }
    }

    fn check_url(&mut self, file: &Utf8Path, line_no: usize, path: &str) -> Option<String> {
        self.count.urls += 1;
        if path.starts_with("http:") || path.starts_with("https:") || path.starts_with("#") {
            return None;
        }
        let path = path.split('#').next().unwrap();

        self.count.paths += 1;

        let resolved = file.parent().unwrap().join(Utf8Path::new(path));
        if !resolved.exists() {
            self.count.missing += 1;
            // println!("{}:{}: {}", file, line_no, path);

            let filenames = &self.filenames;
            if let Some(names) = resolved.file_name().and_then(|f| filenames.get(f)) {
                self.count.matches += 1;
                if names.len() > 1 {
                    self.count.ambiguous += 1;
                }
                if let Some(mut replacement) = Self::rank_names(file, names, &mut self.count) {
                    // mdBook doesn't seem to like "raw" filenames.
                    if !replacement.starts_with("../") {
                        replacement.insert_str(0, "./");
                    }
                    // println!("- Replacing with: {}", replacement);
                    return Some(replacement);
                }
            } else {
                eprintln!("Warning: Unable to resolve at {}:{}: {}", file, line_no, path);
            }
        }
        None
    }

    fn rank_names(file: &Utf8Path, names: &[Utf8PathBuf], count: &mut Counts) -> Option<String> {
        let parent_dir = file.parent().unwrap();
        let mut best_score = usize::MAX;
        let mut best_path = None;
        let mut ties = 0;
        for candidate in names {
            // println!("- Could be: {}", candidate);

            // Candidates are scored by how many differing path components they have.
            if let Some(relpath) = diff_paths(candidate, parent_dir) {
                let score = relpath.components().count() +
                    // Triple count ../ components (prefer candidates in the same directory).
                    relpath.components().take_while(|c| c == &Component::ParentDir).count() * 2;
                if score < best_score {
                    best_score = score;
                    best_path = Some(relpath.into_os_string().into_string().unwrap());
                    ties = 0;
                } else if score == best_score {
                    ties += 1;
                }
            };
        }

        if best_path.is_some() && ties > 0 {
            count.ties += 1;
        }

        best_path
    }
}
