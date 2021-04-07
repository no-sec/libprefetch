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

//! A forensic library which parses and reads Microsoft Prefetch files.
//!
//! `libprefetch` fully supports the following versions of Windows:
//!   * Windows 2003
//!   * Windows XP
//!   * Windows Vista
//!   * Windows 7
//!   * Windows 8/8.1
//!
//! `libprefetch` **partially supports** Windows 10.
//!
//! Features:
//!   * Parser and validator
//!   * Auto detects version of Windows
//!   * Provides the last execution time and the execution counter
//!   * Provides metric information about loaded files (like dll etc) **if available**, such as :
//!     * filename
//!     * start time
//!     * duration
//!     * average duration
//!     * NTFS MFT entry
//!     * NTFS sequence numer
//!   * Provides the trace chains (**unavailable for Windows 10**)
//!   * Provides all pieces of information about the volumes:
//!     * device path
//!     * creation time
//!     * serial number
//!     * list of directories
//!
//! This library will be used in a global forensic computing library very soon.
//!
//! ## Example
//!
//! ```rust
//! use libprefetch::Prefetch;
//!
//! let file = std::fs::File::open("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
//!
//! let prefetch = Prefetch::new(file).unwrap();
//!
//! // Prints some information
//! println!("Executable {} launched {} times. The last time was: {}",
//!   prefetch.name(),
//!   prefetch.execution_counter(),
//!   prefetch.last_execution_time() // TODO: format the FILETIME here
//! );
//!
//! // Iterates over all loaded DLL etc for the prefetch file
//! println!(" ===== File metrics ===== ");
//! for metric in prefetch.metrics().unwrap() {
//!   println!("#{}: {}", metric.id(), metric.filename());
//!   println!("    start time: {}", metric.start_time().unwrap());
//!   println!("    duration: {}", metric.duration().unwrap());
//!   println!(" ------------------------------- ");
//! }
//!
//! // Iterates over the volumes
//! println!(" ===== Volumes ===== ");
//! for volume in prefetch.volumes().unwrap() {
//!   println!("Volume #{}:", volume.id());
//!   println!("    Path: {}", volume.device_path());
//!   println!("    Creation time: {}", volume.creation_time());
//!   println!("    Serial number: {}", volume.serial_number());
//!   println!("    Directories: ");
//!   for directory in volume.directories().unwrap() {
//!     println!("        {}", directory);
//!   }
//! }
//!
//!
//! ```
//!
//! ## Releases
//!
//! Release notes are available in [RELEASES.md](RELEASES.md).
//!
//! ## Compatibility
//!
//! `libprefetch` seems to work for rust 1.9 and greater.
mod prefetch;
mod parser;
mod error;
mod constants;
mod header;
mod util;
pub mod iterator;
pub mod metric;
pub mod trace;
pub mod volume;

pub(crate) use error::Result;
pub use prefetch::{FormatVersion, Prefetch};
pub use error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    fn prelude() -> prefetch::Prefetch {
      let f =
        std::fs::File::open("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
      let p = prefetch::Prefetch::new(f).unwrap();

      p
    }

    #[test]
    fn header() {
      let p = prelude();
      assert_eq!(129453035816965472, p.last_execution_time());
      assert_eq!(38, p.execution_counter());
    }

    #[test]
    fn metrics() {
      let p = prelude();
      for metric in p.metrics().unwrap() {
        println!("metric={:?}", metric);
      }
    }

    #[test]
    fn trace() {
      let p = prelude();
      for trace in p.trace().unwrap() {
        println!("trace={:?}", trace);
      }
    }

    #[test]
    fn volumes() {
      let p = prelude();
      for volume in p.volumes().unwrap() {
        println!("volume={:?}", volume);
      }
    }

    #[test]
    fn volumes_directories() {
      let p = prelude();
      for volume in p.volumes().unwrap() {
        for d in volume.directories().unwrap() {
          println!("directories={:?}", d);
        }
      }
    }

    #[test]
    fn readme() {
      let file = std::fs::File::open("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();

      let prefetch = Prefetch::new(file).unwrap();

      // Prints some information
      println!("Executable {} launched {} times. The last time was: {}",
        prefetch.name(),
        prefetch.execution_counter(),
        prefetch.last_execution_time() // TODO: format the MSTIME here
      );

      // Iterates over all loaded DLL etc for the prefetch file
      println!(" ===== File metrics ===== ");
      for metric in prefetch.metrics().unwrap() {
        println!("#{}: {}", metric.id(), metric.filename());
        println!("    start time: {}", metric.start_time().unwrap());
        println!("    duration: {}", metric.duration().unwrap());
        println!(" ------------------------------- ");
      }

      // Iterates over the volumes
      println!(" ===== Volumes ===== ");
      for volume in prefetch.volumes().unwrap() {
        println!("Volume #{}:", volume.id());
        println!("    Path: {}", volume.device_path());
        println!("    Creation time: {}", volume.creation_time());
        println!("    Serial number: {}", volume.serial_number());
        println!("    Directories: ");
        for directory in volume.directories().unwrap() {
          println!("        {}", directory);
        }
      }
    }
}
