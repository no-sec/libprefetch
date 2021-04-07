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
use super::util::FromSlice;

/// A file metric entry.
///
/// This is supposed to give some information about
/// a file which is load by the exe.
#[derive(Debug)]
pub struct MetricEntry {
  id: usize,
  start_time: Option<u32>,
  duration: Option<u32>,
  average_duration: Option<u32>,
  filename: std::string::String,
  mft_entry_index: Option<u64>,
  sequence_number: Option<u16>
}

impl MetricEntry {

  /// Returns the ID of the entry.
  pub fn id(&self) -> usize {
    self.id
  }

  /// Returns the start time (when the file is loaded).
  pub fn start_time(&self) -> Option<u32> {
    self.start_time
  }

  /// Returns the duration (loading duration).
  pub fn duration(&self) -> Option<u32> {
    self.duration
  }

  /// Returns the average duration.
  pub fn average_duration(&self) -> Option<u32> {
    self.average_duration
  }

  /// Returns the filename.
  pub fn filename(&self) -> &str {
    &self.filename
  }

  /// Returns the NTFS MFT entry index, if available.
  pub fn mft_entry_index(&self) -> Option<u64> {
    self.mft_entry_index
  }

  /// Returns the NTFS sequence number, if available.
  pub fn sequence_number(&self) -> Option<u16> {
    self.sequence_number
  }
}


pub(crate) trait MetricParser {

  fn parse_metrics(&self, content: &[u8])
    -> super::Result<std::vec::Vec<super::metric::MetricEntry>>;
}

impl MetricParser for super::parser::WindowsXp2003 {

  fn parse_metrics(&self, content: &[u8])
    -> super::Result<std::vec::Vec<super::metric::MetricEntry>> {
    let offset = usize::from_slice(&content[0x54 .. 0x58]);
    let n = usize::from_slice(&content[0x58 .. 0x5c]);
    let mut entries = std::vec::Vec::<MetricEntry>::with_capacity(n);
    let entry_size = 20usize;
    let section = &content[offset .. offset + n * entry_size];
    let name_section_offset = usize::from_slice(&content[0x64 .. 0x68]);
    let name_section_length = usize::from_slice(&content[0x68 .. 0x6c]);
    let name_section = &content[name_section_offset .. name_section_offset +
      name_section_length];

    // Each entry is 20 bytes
    for i in 0 .. n {
      let entry = &section[entry_size * i .. entry_size * (i + 1)];

      let name_offset = usize::from_slice(&entry[0x8 .. 0xc]);
      let name_length = usize::from_slice(&entry[0xc .. 0x10]);

      entries.push(MetricEntry {
        id: i,
        start_time: Some(u32::from_slice(&entry[0x0 .. 0x4])),
        duration: Some(u32::from_slice(&entry[0x4 .. 0x8])),
        average_duration: None,
        filename: super::util::fetch_unicode_string(&name_section, name_offset,
        name_length),
        mft_entry_index: None,
        sequence_number: None
      });
    }

    Ok(entries)
  }

}

impl MetricParser for super::parser::WindowsVista7 {

  fn parse_metrics(&self, content: &[u8])
    -> super::Result<std::vec::Vec<super::metric::MetricEntry>> {

    let offset = usize::from_slice(&content[0x54 .. 0x58]);
    let n = usize::from_slice(&content[0x58 .. 0x5c]);
    let mut entries = std::vec::Vec::<MetricEntry>::with_capacity(n);
    let entry_size = 32;
    let section = &content[offset .. offset + n * entry_size];
    let name_section_offset = usize::from_slice(&content[0x64 .. 0x68]);
    let name_section_length = usize::from_slice(&content[0x68 .. 0x6c]);
    let name_section = &content[name_section_offset .. name_section_offset +
      name_section_length];

    for i in 0 .. n {
      let entry = &section[entry_size * i .. entry_size * (i + 1)];

      let name_offset = usize::from_slice(&entry[0xc .. 0x10]);
      let name_length = usize::from_slice(&entry[0x10 .. 0x14]);

      entries.push(MetricEntry {
        id: i,
        start_time: Some(u32::from_slice(&entry[0x0 .. 0x4])),
        duration: Some(u32::from_slice(&entry[0x4 .. 0x8])),
        average_duration: Some(u32::from_slice(&entry[0x8 .. 0xc])),
        filename: super::util::fetch_unicode_string(&name_section, name_offset,
        name_length),
        mft_entry_index: Some(u64::from_slice(&entry[0x18 .. 0x1e])),
        sequence_number: Some(u32::from_slice(&entry[0x1e .. 0x20]) as u16)
      });
    }

    Ok(entries)
  }

}

impl MetricParser for super::parser::Windows8 {

  fn parse_metrics(&self, content: &[u8])
    -> super::Result<std::vec::Vec<super::metric::MetricEntry>> {

    let offset = usize::from_slice(&content[0x54 .. 0x58]);
    let n = usize::from_slice(&content[0x58 .. 0x5c]);
    let mut entries = std::vec::Vec::<MetricEntry>::with_capacity(n);
    let entry_size = 32;
    let section = &content[offset .. offset + n * entry_size];
    let name_section_offset = usize::from_slice(&content[0x64 .. 0x68]);
    let name_section_length = usize::from_slice(&content[0x68 .. 0x6c]);
    let name_section = &content[name_section_offset .. name_section_offset +
      name_section_length];

    // Each entry is 20 bytes
    for i in 0 .. n {
      let entry = &section[entry_size * i .. entry_size * (i + 1)];

      let name_offset = usize::from_slice(&entry[0xc .. 0x10]);
      let name_length = usize::from_slice(&entry[0x10 .. 0x14]);


      entries.push(MetricEntry {
        id: i,
        start_time: Some(u32::from_slice(&entry[0x0 .. 0x4])),
        duration: Some(u32::from_slice(&entry[0x4 .. 0x8])),
        average_duration: Some(u32::from_slice(&entry[0x8 .. 0xc])),
        filename: super::util::fetch_unicode_string(&name_section, name_offset,
        name_length),
        mft_entry_index: Some(u64::from_slice(&entry[0x18 .. 0x1e])),
        sequence_number: Some(u32::from_slice(&entry[0x1e .. 0x20]) as u16)
      });
    }

    Ok(entries)
  }

}

impl MetricParser for super::parser::Windows10 {

  fn parse_metrics(&self, content: &[u8])
    -> super::Result<std::vec::Vec<super::metric::MetricEntry>> {

    let offset = usize::from_slice(&content[0x54 .. 0x58]);
    let n = usize::from_slice(&content[0x58 .. 0x5c]);
    let mut entries = std::vec::Vec::<MetricEntry>::with_capacity(n);
    let entry_size = 32;
    let section = &content[offset .. offset + n * entry_size];
    let name_section_offset = usize::from_slice(&content[0x64 .. 0x68]);
    let name_section_length = usize::from_slice(&content[0x68 .. 0x6c]);
    let name_section = &content[name_section_offset .. name_section_offset +
      name_section_length];

    // Each entry is 20 bytes
    for i in 0 .. n {
      let entry = &section[entry_size * i .. entry_size * (i + 1)];

      let name_offset = usize::from_slice(&entry[0xc .. 0x10]);
      let name_length = usize::from_slice(&entry[0x10 .. 0x14]);


      entries.push(MetricEntry {
        id: i,
        start_time: Some(u32::from_slice(&entry[0x0 .. 0x4])),
        duration: Some(u32::from_slice(&entry[0x4 .. 0x8])),
        average_duration: Some(u32::from_slice(&entry[0x8 .. 0xc])),
        filename: super::util::fetch_unicode_string(&name_section, name_offset,
        name_length),
        mft_entry_index: Some(u64::from_slice(&entry[0x18 .. 0x1e])),
        sequence_number: Some(u32::from_slice(&entry[0x1e .. 0x20]) as u16)
      });
    }

    Ok(entries)
  }

}
