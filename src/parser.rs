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

pub(crate) struct ParserResult {
  pub(crate) last_execution_time: u64,
  pub(crate) execution_counter: usize,
  pub(crate) metrics:
    super::Result<std::vec::Vec<super::metric::MetricEntry>>,
  pub(crate) trace:
    super::Result<std::vec::Vec<super::trace::TraceEntry>>,
  pub(crate) volumes:
    super::Result<std::vec::Vec<super::volume::VolumeEntry>>
}

pub(crate) trait Parser: super::metric::MetricParser
  + super::trace::TraceParser
  + super::volume::VolumeParser {

  fn parse(&self, content: &[u8]) -> super::Result<ParserResult>;
}
use super::metric::MetricParser;
use super::trace::TraceParser;
use super::volume::VolumeParser;

pub(crate) struct WindowsXp2003;
pub(crate) struct WindowsVista7;
pub(crate) struct Windows8;
pub(crate) struct Windows10;



impl Parser for WindowsXp2003 {
  fn parse(&self, content: &[u8]) -> super::Result<ParserResult> {
    Ok(ParserResult {
      last_execution_time: u64::from_slice(&content[0x78 .. 0x80]),
      execution_counter: usize::from_slice(&content[0x90 .. 0x94]),
      metrics: self.parse_metrics(content),
      trace: self.parse_trace(content),
      volumes: self.parse_volumes(content)
    })
  }

}
impl Parser for WindowsVista7 {
  fn parse(&self, content: &[u8]) -> super::Result<ParserResult> {
    Ok(ParserResult {
      last_execution_time: u64::from_slice(&content[0x80 .. 0x88]),
      execution_counter: usize::from_slice(&content[0x98 .. 0x9c]),
      metrics: self.parse_metrics(content),
      trace: self.parse_trace(content),
      volumes: self.parse_volumes(content)
    })
  }

}
impl Parser for Windows8 {
  fn parse(&self, content: &[u8]) -> super::Result<ParserResult> {
    Ok(ParserResult {
      last_execution_time: u64::from_slice(&content[0x80 .. 0x88]),
      execution_counter: usize::from_slice(&content[0xd0 .. 0xd4]),
      metrics: self.parse_metrics(content),
      trace: self.parse_trace(content),
      volumes: self.parse_volumes(content)
    })
  }
}
impl Parser for Windows10 {
  fn parse(&self, _content: &[u8]) -> super::Result<ParserResult> {
    Err(super::error::Error::NotImplemented)
  }
}
