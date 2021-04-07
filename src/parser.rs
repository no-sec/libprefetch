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
