/*
    Provides a bodyfile reader for the prefetch hash cracker
    Author: Harel Segev
    06/10/2022
*/

use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader, Lines, ErrorKind};
use itertools::Itertools;


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
    type Item = io::Result<String>;

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
                    return match line.split("|").next_tuple() {
                        Some((_, filename, metadata_address)) => {
                            let attribute_type = metadata_address.split("-").nth(1);

                            if attribute_type != Some("144") {
                                continue;
                            }

                            let res = strip_mount_point(self.mount_point, filename)
                                .and_then(|res| strip_deleted(res));

                            Some(
                                match res {
                                    Some(res) =>
                                        Ok(res.replace("/", "\\").to_uppercase()),

                                    None =>
                                        Err(
                                            io::Error::new(
                                                ErrorKind::InvalidData,
                                                "could not strip mount point"
                                            )
                                        )
                                }
                            )
                        }

                        None => Some(
                            Err(
                                io::Error::new(
                                    ErrorKind::InvalidData,
                                    "invalid format"
                                )
                            )
                        )
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