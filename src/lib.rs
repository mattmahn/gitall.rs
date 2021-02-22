/*!
Crate `gitall` provides an simple API for recursively finding all Git
directories below a given directory.

# Example

The following code prints the directory paths of each Git directory located
beneath the current directory:
```
use gitall::Giterator;
use std::default::Default;

let giterator = Giterator::default();
for direntry in giterator {
    println!("{}", direntry.path().display());
}
```

This snippet only finds Git directories whose name matches a regular expression:
```
use gitall::Giterator;
use regex::Regex;
use std::default::Default;

let giterator = Giterator::default().regex(Regex::new(r"some-pattern-.+").unwrap());
for direntry in giterator {
    println!("{}", direntry.path().display());
}
```

Check the [`Giterator`][] documentation for more ways to filter results.
*/

use regex::Regex;
use walkdir::{DirEntry, WalkDir};

use std::{
    default::Default,
    iter,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct GiteratorOptions {
    root: PathBuf,
    follow_links: bool,
    max_depth: usize,
    regex: Regex,
    match_full_path: bool,
}

/// A builder to define how to search for Git directories.
#[derive(Debug)]
pub struct Giterator {
    options: GiteratorOptions,
}

impl Giterator {
    /// Sets the apex root directory to begin searching. The directory itself
    /// will not be considered for inclusion in the final results; only
    /// directories within `root`.
    pub fn root<P: AsRef<Path>>(mut self, root: P) -> Self {
        self.options.root = root.as_ref().to_path_buf();
        self
    }

    /// Sets wether or not to follow symbolic links in the directory tree.
    pub fn follow_links(mut self, follow: bool) -> Self {
        self.options.follow_links = follow;
        self
    }

    /// Sets the maximum number of directories from `root` to search. Directories
    /// more than `depth` directories deeper than `root` will not be found.
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.options.max_depth = depth;
        self
    }

    /// Sets the regular expression that must match directory paths.
    pub fn regex(mut self, regex: Regex) -> Self {
        self.options.regex = regex;
        self
    }

    /// Sets wether or not to match `regex` against the full
    /// [canonicalized][canonical] directory path.
    ///
    /// [canonical]: std::path::Path::canonicalize
    pub fn match_full_path(mut self, match_full_path: bool) -> Self {
        self.options.match_full_path = match_full_path;
        self
    }
}

impl Default for Giterator {
    /// Creates a default Giterator with the following settings:
    ///   - search the current working directory
    ///   - do not follow symbolic links
    ///   - search [`usize::MAX`][] directories deep
    ///   - match any directory path
    fn default() -> Self {
        Giterator {
            options: GiteratorOptions {
                root: Path::new(".").to_path_buf(),
                follow_links: false,
                max_depth: usize::MAX,
                regex: Regex::new(".*").unwrap(),
                match_full_path: false,
            },
        }
    }
}

impl iter::IntoIterator for Giterator {
    type Item = DirEntry;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            walker: WalkDir::new(self.options.root).into_iter(),
            regex: self.options.regex,
            full_path: self.options.match_full_path,
        }
    }
}

#[derive(Debug)]
/// An iterator for recursively finding Git directories.
pub struct IntoIter {
    walker: ::walkdir::IntoIter,
    regex: Regex,
    full_path: bool,
}

impl iter::Iterator for IntoIter {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = match self.walker.next() {
                None => return None, // no more directories to walk
                Some(Err(_err)) => {
                    // TODO provide a debug log message
                    self.walker.skip_current_dir();
                    continue;
                }
                Some(Ok(entry)) => entry,
            };
            if is_git_dir(&entry) {
                self.walker.skip_current_dir();
                if is_matching_dir(&entry, &self.regex, self.full_path) {
                    return Some(entry);
                }
            }
        }
    }
}

/// Returns true if the directory represented by `entry` is a Git directory root.
pub fn is_git_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.path().join(".git").is_dir()
}

/// Returns true if the directory represented by `entry` matches the given
/// regular express. If `full_path` is true, the regex matches against the
/// [canonicalized][canonical] path of `entry`.
///
/// [canonical]: std::path::Path::canonicalize
fn is_matching_dir(entry: &DirEntry, regex: &Regex, full_path: bool) -> bool {
    let canonical_path = entry
        .path()
        .canonicalize()
        .expect("failed to canonicalize the directory path");
    let pathname: &str = if full_path {
        canonical_path.to_str().unwrap()
    } else {
        entry.file_name().to_str().unwrap()
    };
    regex.is_match(pathname)
}
