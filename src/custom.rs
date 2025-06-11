use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufReader, Cursor, Read},
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use anyhow::{ensure, Context, Result};
use log::debug;
use ureq::tls::{RootCerts, TlsConfig, TlsProvider};
use ureq::Agent;
use walkdir::{DirEntry, WalkDir};
use zip::ZipArchive;

use crate::{config::TlsBackend, types::PlatformType, utils::print_warning};

pub fn to_stem_custom(entry: DirEntry) -> Option<String> {
    entry
        .path()
        .file_name()
        .and_then(OsStr::to_str)
        .and_then(|s| {
            if s.ends_with(".page.md") {
                s.strip_suffix(".page.md")
            } else {
                s.strip_suffix(".patch.md")
            }
        })
        .map(str::to_string)
}

pub fn custom_pages(custom_pages_dir: &Path) -> impl Iterator<Item = String> {
    let is_page = |entry: &DirEntry| -> bool {
        entry.file_type().is_file()
            && entry
                .path()
                .file_name()
                .and_then(OsStr::to_str)
                .is_some_and(|file_name| {
                    file_name.ends_with(".page.md") || file_name.ends_with(".patch.md")
                })
    };

    WalkDir::new(custom_pages_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(is_page)
        .filter_map(Result::ok)
        .filter_map(to_stem_custom)
}
