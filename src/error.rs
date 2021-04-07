// This file is part of libprefetch.
//
// libprefetch is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// libprefetch is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with libprefetch.  If not, see <http://www.gnu.org/licenses/>.
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