/*
    Provides a bodyfile reader for the prefetch hash cracker
    Author: Harel Segev
    06/10/2022
*/

use std::fs::File;
use std::io::{prelude::*, BufReader, Lines, ErrorKind};


pub struct BodyfileReader<'a> {
    lines: Lines<BufReader<File>>,
    mount_point: &'a str
}

impl<'a> BodyfileReader<'a> {
    pub fn new(bodyfile: File, mount_point: &'a str) -> Self {
        let reader = BufReader::new(bodyfile);
        let lines = reader.lines();

        Self {
            lines,
            mount_point
        }
    }
}

impl<'a> Iterator for BodyfileReader<'a> {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.lines.next()?;

            match line {
                Err(error) => {
                    match error.kind() {
                        ErrorKind::InvalidData => continue,
                        _ => return Some(Err(error)),
                    }
                },

                Ok(line) => {
                    let attribute_type = line
                        .split("|")
                        .nth(2)
                        .and_then(|metadata_address| {
                            metadata_address.split("-").nth(1)
                        });

                    if attribute_type != Some("144") {
                        continue;
                    }

                    let res = line
                        .split("|")
                        .nth(1)

                        .and_then(|res| strip_mount_point(self.mount_point, res))
                        .and_then(|res| strip_deleted(res));

                    if let Some(res) = res {
                        return Some(
                            Ok(res.replace("/", "\\").to_uppercase())
                        );
                    }
                }
            }
        }
    }
}

fn strip_mount_point<'a>(mount_point: &str, path: &'a str) -> Option<&'a str> {
    path.strip_prefix(mount_point).and_then(|res| {
        if mount_point.ends_with("/") {
            Some(res)
        }
        else {
            res.strip_prefix("/")
        }
    })
}

fn strip_deleted(path: &str) -> Option<&str> {
    path.strip_suffix(" (deleted)").or(Some(path))
}