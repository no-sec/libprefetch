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

pub(crate) struct Header {
  pub(crate) version: super::prefetch::FormatVersion,
  pub(crate) size: usize,
  pub(crate) name: std::string::String,
  pub(crate) hash: u32
}

impl Header {

  pub(crate) fn new(content: &[u8]) -> super::Result<(Header,
  Box<super::parser::Parser>)> {
    use super::util::FromSlice;
    let result: super::Result<(Header, Box<super::parser::Parser>)>;
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
