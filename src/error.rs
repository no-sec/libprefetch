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
// Author: zadig <thomas chr(0x40) bailleux.me>

use std;

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

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::UnknownFormatVersion(_v) => "Unknown prefetch format version.",
      Error::NotPrefetchFile => "This is not a prefetch file",
      Error::IOError(ref e) => e.description(),
      Error::NotImplemented => "Not implemented yet"
    }
  }

  fn cause(&self) -> Option<&std::error::Error> {
    match *self {
      Error::IOError(ref e) => Some(e),
      _ => None
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use std::error::Error;
    write!(f, "{}", self.description())
  }
}
