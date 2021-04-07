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

pub(crate) struct Header {
  pub(crate) version: super::prefetch::FormatVersion,
  pub(crate) size: usize,
  pub(crate) name: std::string::String,
  pub(crate) hash: u32
}

impl Header {

  pub(crate) fn new(content: &[u8]) -> super::Result<(Header,
  Box<dyn super::parser::Parser>)> {
    use super::util::FromSlice;
    let result: super::Result<(Header, Box<dyn super::parser::Parser>)>;
    if content.len() < super::constants::HEADER_LENGTH {
      result = Err(super::error::Error::NotPrefetchFile);
    } else {
      let (version, parser) = super::prefetch::FormatVersion::new(
        u32::from_slice(&content[0x0 .. 0x4]))?;
      if &content[0x4 .. 0x8] != &super::constants::HEADER_CONSTANT_FIELD {
        result = Err(super::error::Error::NotPrefetchFile);
      } else {
        let size = usize::from_slice(&content[0xc .. 0x10]);
        let mut name =std::string::String::with_capacity(
            super::constants::MAX_FILENAME_LENGTH);
        let mut i = 0;
        let limit = (super::constants::MAX_FILENAME_LENGTH + 1) * 2;
        let name_slice = &content[0x10 .. 0x4c];
        while i < limit && name_slice[i] != 0 {
          name.push(name_slice[i] as char);
          i += 2;
        }
        let hash = u32::from_slice(&content[0x4c .. 0x50]);
        result = Ok((Header {
          version: version,
          size: size,
          name: name,
          hash: hash
        }, parser));
      }
    }

    result
  }
}
