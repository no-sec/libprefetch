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

pub(crate) trait FromSlice<T> {
  fn from_slice(buf: &[T]) -> Self;
}


impl FromSlice<u8> for usize {
  fn from_slice(buf: &[u8]) -> Self {
    let mut result = 0usize;
    let mut p = 0u32;
    for i in 0..buf.len() {
      result += (buf[i] as usize) * 256usize.pow(p);
      p += 1;
    }
    result
  }
}

impl FromSlice<u8> for u32 {
  fn from_slice(buf: &[u8]) -> Self {
    let mut result = 0u32;
    let mut p = 0u32;
    for i in 0..buf.len() {
      result += (buf[i] as u32) * 256u32.pow(p);
      p += 1;
    }
    result
  }
}

impl FromSlice<u8> for i32 {
  fn from_slice(buf: &[u8]) -> Self {
    let mut result = 0i32;
    let mut p = 0u32;
    for i in 0..buf.len() {
      result += (buf[i] as i32) * 256i32.pow(p);
      p += 1;
    }
    result
  }
}

impl FromSlice<u8> for u64 {
  fn from_slice(buf: &[u8]) -> Self {
    let mut result = 0u64;
    let mut p = 0u32;
    for i in 0..buf.len() {
      result += (buf[i] as u64) * 256u64.pow(p);
      p += 1;
    }
    result
  }
}

pub(crate) fn fetch_unicode_string(content: &[u8], offset: usize, max_length:
  usize) -> std::string::String {
  let mut s = std::string::String::new();
  let mut i = 0;
  let limit = content.len();
  let limit2 = max_length * 2 + 2;
  while i < limit2 && i < limit && content[offset + i] != 0 {
    s.push(content[offset + i] as char);
    i += 2;
  }

  s
}

