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

pub(crate) const FORMAT_WINDOWS_XP_2003: u32 = 0x11;
pub(crate) const FORMAT_WINDOWS_VISTA_7: u32 = 0x17;
pub(crate) const FORMAT_WINDOWS_8: u32 = 0x1a;
pub(crate) const FORMAT_WINDOWS_10: u32 = 0x1e;

pub(crate) const HEADER_LENGTH: usize = 0x54;
pub(crate) const HEADER_CONSTANT_FIELD: [u8; 4] = [0x53, 0x43, 0x43, 0x41];
pub(crate) const MAX_FILENAME_LENGTH: usize = 29;
