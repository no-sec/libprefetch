//             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2018 Thomas Bailleux <thomas@bailleux.me>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.
//
// Authors: zadig <thomas chr(0x40) bailleux.me>
//          jasa <jan.starke (0x40) t-systems.com>

use std;
use std::fmt;

/// Errors related to the process of parsing and reading.
#[derive(Debug)]
pub enum Error {

  /// Unknown Prefetch file version.
  UnknownFormatVersion(u32),

  /// Not a prefetch file (invalid header etc).
  NotPrefetchFile,

  /// Basic IO error.
  IOError(std::io::Error),

  /// Not supported or not implemented yet (like for Windows 10).
  NotImplemented
}

/// Classic custom Result type.
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::UnknownFormatVersion(_v) => write!(f, "Unknown prefetch format version."),
      Error::NotPrefetchFile => write!(f, "This is not a prefetch file"),
      Error::IOError(ref e) => e.fmt(f),
      Error::NotImplemented => write!(f, "Not implemented yet")
  }
  }
}
impl std::error::Error for Error {
  #[allow(deprecated, deprecated_in_future)]
  fn description(&self) -> &str {
    match *self {
      Error::UnknownFormatVersion(_v) => "Unknown prefetch format version.",
      Error::NotPrefetchFile => "This is not a prefetch file",
      Error::IOError(ref e) => e.description(),
      Error::NotImplemented => "Not implemented yet"
    }
  }

  fn cause(&self) -> Option<&dyn std::error::Error> {
    match *self {
      Error::IOError(ref e) => Some(e),
      _ => None
    }
  }
}