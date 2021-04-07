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

/// Format versions for a Prefetch file.
#[derive(Debug, Clone, Copy)]
pub enum FormatVersion {

  /// Windows XP and Windows 2003 (code 0x11)
  WindowsXp2003,

  /// Windows Vista and Windows 7 (code 0x17)
  WindowsVista7,

  /// Windows 8 and Windows 8.1 (code 0x1a)
  Windows8,

  /// Windows 10 (code 0x1e) (not fully supported yet)
  Windows10
}

impl FormatVersion {

  pub(crate) fn new(value: u32)
      -> super::Result<(FormatVersion, Box<dyn super::parser::Parser>)> {
    match value {
      super::constants::FORMAT_WINDOWS_XP_2003
        => Ok((FormatVersion::WindowsXp2003,
        Box::new(super::parser::WindowsXp2003{}))),

      super::constants::FORMAT_WINDOWS_VISTA_7
        => Ok((FormatVersion::WindowsVista7,
        Box::new(super::parser::WindowsVista7{}))),

      super::constants::FORMAT_WINDOWS_8
        => Ok((FormatVersion::Windows8,
        Box::new(super::parser::Windows8{}))),

      super::constants::FORMAT_WINDOWS_10
        => Ok((FormatVersion::Windows10,
        Box::new(super::parser::Windows10{}))),

      _ => Err(super::error::Error::UnknownFormatVersion(value))
    }
  }
}

impl std::fmt::Display for FormatVersion {

  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      FormatVersion::WindowsXp2003 => write!(f, "Windows XP or 2003"),
      FormatVersion::WindowsVista7 => write!(f, "Windows Vista or 7"),
      FormatVersion::Windows8 => write!(f, "Windows 8 or 8.1"),
      FormatVersion::Windows10 => write!(f, "Windows 10")
    }
  }
}


/// A Prefetch file reader and parser.
///
/// # Basic Example
///
/// ```
/// use libprefetch::Prefetch;
///
///
/// let file = std::fs::File::open("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
///
/// let prefetch = Prefetch::new(file).unwrap();
///
/// // Or with a path
///
/// let prefetch =
///      Prefetch::from_path("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
///
/// ```

pub struct Prefetch {
  header: super::header::Header,
  //parser: Box<super::parser::Parser>,
  parser_result: super::parser::ParserResult
}

impl Prefetch {

  /// Returns the version of the Prefetch file.
  pub fn version(&self) -> FormatVersion {
    self.header.version
  }

  /// Returns the size of the Prefetch file.
  pub fn size(&self) -> usize {
    self.header.size
  }

  /// Returns the name of the executable.
  pub fn name(&self) -> &str {
    &self.header.name
  }

  /// Returns the prefetch hash of the executable.
  pub fn hash(&self) -> u32 {
    self.header.hash
  }

  /// Returns the last execution time, in FILETIME format
  pub fn last_execution_time(&self) -> u64 {
    self.parser_result.last_execution_time
  }

  /// Returns the execution counter (how many times the exe was run).
  pub fn execution_counter(&self) -> usize {
    self.parser_result.execution_counter
  }

  /// Returns an Iterator for file metrics.
  ///
  /// # Example
  ///
  /// ```
  /// use libprefetch::Prefetch;
  ///
  /// let prefetch =
  ///     Prefetch::from_path("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
  ///
  /// // Iterate over metrics
  /// for metric in prefetch.metrics().unwrap() {
  ///     println!("Loaded file by the exe: {}", metric.filename());
  /// }
  /// ```
  pub fn metrics(&self)
      -> super::Result<super::iterator::MetricIterator> {
    super::iterator::MetricIterator::new(&self.parser_result)
  }


  /// Returns an Iterator for the trace chains.
  ///
  /// # Example
  ///
  /// ```
  /// use libprefetch::Prefetch;
  ///
  /// let prefetch =
  ///     Prefetch::from_path("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
  ///
  /// // Iterate over the chain
  /// for item in prefetch.trace().unwrap() {
  ///     println!("Trace #{}, bytes loaded:: {}", item.id(), item.load_count());
  /// }
  /// ```
  pub fn trace(&self)
      -> super::Result<super::iterator::TraceIterator> {
    super::iterator::TraceIterator::new(&self.parser_result)
  }

  /// Returns an Iterator for the volumes.
  ///
  /// # Example
  ///
  /// ```
  /// use libprefetch::Prefetch;
  ///
  /// let prefetch =
  ///     Prefetch::from_path("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
  ///
  /// // Iterates over the volumes
  /// println!(" ===== Volumes ===== ");
  /// for volume in prefetch.volumes().unwrap() {
  ///   println!("Volume #{}:", volume.id());
  ///   println!("    Path: {}", volume.device_path());
  ///   println!("    Creation time: {}", volume.creation_time());
  ///   println!("    Serial number: {}", volume.serial_number());
  ///   println!("    Directories: ");
  ///   for directory in volume.directories().unwrap() {
  ///     println!("        {}", directory);
  ///   }
  /// }
  ///
  /// ```
  pub fn volumes(&self)
      -> super::Result<super::iterator::VolumeIterator> {
    super::iterator::VolumeIterator::new(&self.parser_result)
  }

  /// Constructs a new `Prefetch` from a `std::io::Read` source.
  ///
  ///
  /// # Example
  ///
  /// ```rust
  /// use libprefetch::Prefetch;
  ///
  /// let file = std::fs::File::open("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
  ///
  /// let prefetch = Prefetch::new(file).unwrap();
  ///
  /// ```
  pub fn new<T>(mut src: T) -> super::Result<Prefetch>
    where T: std::io::Read {
      let mut buf = Vec::new();
      src.read_to_end(&mut buf).map_err(super::error::Error::IOError)?;
      if &buf[0..4] == b"MAM\x04" {

        let uncompressed_length = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        println!("uncompressed length: {:08x}", uncompressed_length);

        buf = match lzxpress::data::decompress(&buf[8..]) {
          Ok(b) => b,
          Err(e)  => return Err(super::error::Error::LZXPressError(e))
        };
      }

      let header = &buf[0..super::constants::HEADER_LENGTH];
      println!("header length: {}", header.len());

      let (header, parser) = super::header::Header::new(header)?;

      let data = &buf[..];
      println!("data length: {}", data.len());
      let result = parser.parse(&data)?;
    Ok(Prefetch {
      header: header,
      parser_result: result
    })
  }

  /// Constructs a new `Prefetch` from a file path.
  ///
  ///
  /// # Example
  ///
  /// ```rust
  /// use libprefetch::Prefetch;
  ///
  /// let prefetch
  ///      = Prefetch::from_path("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();
  ///
  /// ```
  pub fn from_path(path: &str) -> super::Result<Prefetch> {
    let f = std::fs::File::open(path).map_err(super::error::Error::IOError)?;
    Prefetch::new(f)
  }
}
