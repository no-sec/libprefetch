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
use super::util::FromSlice;

/// A volume.
///
/// When the user launches the application, the prefetch file stores from
/// which volume the exec was launched.
///
/// For example, if you copies an exe into an USB stick, and then you launch it,
/// you'll see here a new entry with details about your USB stick.
///
/// If all files are on C:, you will have only one single entry.
#[derive(Debug)]
pub struct VolumeEntry {
  id: usize,
  device_path: std::string::String,
  creation_time: u64,
  serial_number: u32,
  pub(crate) directories: std::vec::Vec<std::string::String>
}

impl VolumeEntry {

  /// Returns the ID of the entry.
  pub fn id(&self) -> usize {
    self.id
  }

  /// Returns the path to the device.
  pub fn device_path(&self) -> &str {
    &self.device_path
  }

  /// Returns the creation time.
  pub fn creation_time(&self) -> u64 {
    self.creation_time
  }

  /// Returns the serial number of the volume.
  pub fn serial_number(&self) -> u32 {
    self.serial_number
  }

  /// Returns an iterator through all directories used on the volume.
  pub fn directories(&self)
      -> super::Result<super::iterator::DirectoryIterator> {
    super::iterator::DirectoryIterator::new(self)
  }

}

fn fetch_directories(content: &[u8], offset: usize, n: usize)
    -> std::vec::Vec<std::string::String> {
  let mut directories = std::vec::Vec::<std::string::String>::with_capacity(n);

  let section_offset = usize::from_slice(&content[0x6c .. 0x70]);
  let section = &content[section_offset .. ];

  let mut start_offset = offset;
  for _ in 0 .. n {
    let str_size = usize::from_slice(
        &section[start_offset .. start_offset + 2]);
    let start = start_offset + 2;
    let end = start + str_size * 2 + 2;
    let mut i = 0;
    let mut s = std::string::String::with_capacity(str_size);
    while i < (str_size + 1) * 2 && section[start + i] != 0 {
      s.push(section[start + i] as char);
      i += 2;
    }
    directories.push(s);
    start_offset = end;
  }
  directories

}

fn generic(content: &[u8], entry_size: usize)
    -> std::vec::Vec<VolumeEntry> {

  let offset = usize::from_slice(&content[0x6c .. 0x70]);
  let n = usize::from_slice(&content[0x70 .. 0x74]);
  let section = &content[offset .. ];
  let mut entries = std::vec::Vec::<VolumeEntry>::with_capacity(n);
  for i in 0 .. n {
    let entry = &section[i * entry_size .. (i + 1) * entry_size];
    entries.push(VolumeEntry {
      id: i,
      device_path: super::util::fetch_unicode_string(section,
        usize::from_slice(&entry[0x0 .. 0x4]),
        usize::from_slice(&entry[0x4 .. 0x8])
      ),
      creation_time: u64::from_slice(&entry[0x8 .. 0x10]),
      serial_number: u32::from_slice(&entry[0x10 .. 0x14]),
      directories: fetch_directories(content,
        usize::from_slice(&entry[0x1c .. 0x20]),
        usize::from_slice(&entry[0x20 .. 0x24])),
    });
  }

  entries
}

pub(crate) trait VolumeParser {

  fn parse_volumes(&self, content: &[u8])
    -> super::Result<std::vec::Vec<VolumeEntry>>;
}

impl VolumeParser for super::parser::WindowsXp2003 {
  fn parse_volumes(&self, content: &[u8])
    -> super::Result<std::vec::Vec<VolumeEntry>> {
    Ok(generic(content, 40))
  }
}

impl VolumeParser for super::parser::WindowsVista7 {
  fn parse_volumes(&self, content: &[u8])
    -> super::Result<std::vec::Vec<VolumeEntry>> {
    Ok(generic(content, 104))
  }
}

impl VolumeParser for super::parser::Windows8 {
  fn parse_volumes(&self, content: &[u8])
    -> super::Result<std::vec::Vec<VolumeEntry>> {
    Ok(generic(content, 104))
  }
}

impl VolumeParser for super::parser::Windows10 {
  fn parse_volumes(&self, content: &[u8])
    -> super::Result<std::vec::Vec<VolumeEntry>> {
    Ok(generic(content, 96))
  }
}
