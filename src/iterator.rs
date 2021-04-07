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


/// Iterator for file metrics.
///
/// This iterates through all `MetricEntry`.
pub struct MetricIterator<'p> {
  parser_result: &'p super::parser::ParserResult,
  curr: usize

}

impl<'p> MetricIterator<'p> {

  pub(crate) fn new(parser_result: &'p super::parser::ParserResult)
      -> super::Result<MetricIterator> {
    if parser_result.metrics.is_ok() {
      Ok(MetricIterator {
        parser_result: parser_result,
        curr: 0
      })
    } else {
      Err(super::error::Error::NotImplemented)
    }
  }
}

impl<'p> Iterator for MetricIterator<'p> {
  type Item = &'p super::metric::MetricEntry;

  fn next(&mut self) -> Option<Self::Item> {
    match self.parser_result.metrics {
      Ok(ref v) => {
        if self.curr < v.len() {
          self.curr += 1;
          Some(&v[self.curr - 1])
        } else {
          None
        }
      },
      Err(ref _e) => None
    }
  }
}


/// Iterator for trace.
///
/// This iterates through all `TraceEntry`.
pub struct TraceIterator<'p> {
  parser_result: &'p super::parser::ParserResult,
  curr: usize
}

impl<'p> TraceIterator<'p> {

  pub(crate) fn new(parser_result: &'p super::parser::ParserResult)
      -> super::Result<TraceIterator> {
    if parser_result.trace.is_ok() {
      Ok(TraceIterator {
        parser_result: parser_result,
        curr: 0
      })
    } else {
      Err(super::error::Error::NotImplemented)
    }
  }
}

impl<'p> Iterator for TraceIterator<'p> {
  type Item = &'p super::trace::TraceEntry;

  fn next(&mut self) -> Option<Self::Item> {
    match self.parser_result.trace {
      Ok(ref v) => {
        if self.curr < v.len() {
          self.curr += 1;
          Some(&v[self.curr - 1])
        } else {
          None
        }
      },
      Err(ref _e) => None
    }
  }
}

/// Iterator for volumes.
///
/// This iterates through all `VolumeEntry`.
pub struct VolumeIterator<'p> {
  parser_result: &'p super::parser::ParserResult,
  curr: usize
}

impl<'p> VolumeIterator<'p> {

  pub(crate) fn new(parser_result: &'p super::parser::ParserResult)
      -> super::Result<VolumeIterator> {
    if parser_result.volumes.is_ok() {
      Ok(VolumeIterator {
        parser_result: parser_result,
        curr: 0
    })
    } else {
      Err(super::error::Error::NotImplemented)
    }
  }
}

impl<'p> Iterator for VolumeIterator<'p> {
  type Item = &'p super::volume::VolumeEntry;

  fn next(&mut self) -> Option<Self::Item> {
    match self.parser_result.volumes {
      Ok(ref v) => {
        if self.curr < v.len() {
          self.curr += 1;
          Some(&v[self.curr - 1])
        } else {
          None
        }
      },
      Err(ref _e) => None
    }
  }
}


/// Iterator for directories inside a volume.
///
/// This iterates through all `DirectoryEntry::directories`.
pub struct DirectoryIterator<'ve> {
  volume_entry: &'ve super::volume::VolumeEntry,
  curr: usize
}

impl<'ve> DirectoryIterator<'ve> {

  pub(crate) fn new(volume_entry: &'ve super::volume::VolumeEntry)
      -> super::Result<DirectoryIterator> {
    Ok(DirectoryIterator {
      volume_entry: volume_entry,
      curr: 0
    })
  }
}

impl<'ve> Iterator for DirectoryIterator<'ve> {
  type Item = &'ve str;

  fn next(&mut self) -> Option<Self::Item> {
    if self.curr < self.volume_entry.directories.len() {
      self.curr += 1;
      Some(&self.volume_entry.directories[self.curr - 1])
    } else {
      None
    }
  }
}
