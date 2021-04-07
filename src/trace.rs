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

/// An entry for the trace chain.
#[derive(Debug)]
pub struct TraceEntry {
  id: usize,
  next_entry_id: Option<usize>,
  load_count: usize
}

impl TraceEntry {

  /// Returns the ID of the next entry if exists.
  pub fn next_entry_id(&self) -> Option<usize> {
    self.next_entry_id
  }

  /// Returns the ID.
  pub fn id(&self) -> usize {
    self.id
  }

  /// Returns how many bytes are loaded.
  pub fn load_count(&self) -> usize {
    self.load_count
  }
}

pub(crate) trait TraceParser {

  fn parse_trace(&self, content: &[u8])
      -> super::Result<std::vec::Vec<TraceEntry>> {
    let offset = usize::from_slice(&content[0x5c .. 0x60]);
    let n = usize::from_slice(&content[0x60 .. 0x64]);
    let mut entries = std::vec::Vec::<TraceEntry>::with_capacity(n);
    let entry_size = 12usize;
    let section = &content[offset .. offset + n * entry_size];

    for i in 0 .. n {
      let entry = &section[entry_size * i .. entry_size * (i + 1)];
      entries.push(TraceEntry {
        id: i,
        next_entry_id: match usize::from_slice(&entry[0 .. 0x4]) {
          0xFFFFFFFF => None,
          a => Some(a)
        },
        load_count: usize::from_slice(&entry[0x4 .. 0x8])
      });
    }

    Ok(entries)
  }
}

impl TraceParser for super::parser::WindowsXp2003{}
impl TraceParser for super::parser::WindowsVista7{}
impl TraceParser for super::parser::Windows8{}

impl TraceParser for super::parser::Windows10 {

  fn parse_trace(&self, _content: &[u8])
      -> super::Result<std::vec::Vec<TraceEntry>> {
    Err(super::error::Error::NotImplemented)
  }

}
