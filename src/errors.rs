/*
    A custom error type for prefetch hash cracker
    Author: Harel Segev
    06/14/2022
 */

use std::io;
use std::fmt;
use std::num;
use std::error;

#[derive(Debug)]
pub enum PrefetchHashCrackerError {
    BadHash(num::ParseIntError),
    BodyfileError(io::Error)
}

impl fmt::Display for PrefetchHashCrackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrefetchHashCrackerError::BadHash(..) =>
                write!(f, "the provided prefetch hash could not be parsed into int"),

            PrefetchHashCrackerError::BodyfileError(ref e) => {
                match e.kind() {
                    io::ErrorKind::NotFound =>
                        write!(f, "the provided bodyfile could not be found"),

                    _ =>
                        write!(
                            f,
                            "an error occurred while processing the provided bodyfile: {}",
                            e.to_string()
                        )
                }
            }
        }
    }
}

impl error::Error for PrefetchHashCrackerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            PrefetchHashCrackerError::BadHash(ref e) => Some(e),
            PrefetchHashCrackerError::BodyfileError(ref e) => Some(e),
        }
    }
}

impl From<num::ParseIntError> for PrefetchHashCrackerError {
    fn from(error: num::ParseIntError) -> PrefetchHashCrackerError {
        PrefetchHashCrackerError::BadHash(error)
    }
}

impl From<io::Error> for PrefetchHashCrackerError {
    fn from(error: io::Error) -> PrefetchHashCrackerError {
        PrefetchHashCrackerError::BodyfileError(error)
    }
}