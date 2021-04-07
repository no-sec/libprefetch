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

pub(crate) const FORMAT_WINDOWS_XP_2003: u32 = 0x11;
pub(crate) const FORMAT_WINDOWS_VISTA_7: u32 = 0x17;
pub(crate) const FORMAT_WINDOWS_8: u32 = 0x1a;
pub(crate) const FORMAT_WINDOWS_10: u32 = 0x1e;

pub(crate) const HEADER_LENGTH: usize = 0x54;
pub(crate) const HEADER_CONSTANT_FIELD: [u8; 4] = [0x53, 0x43, 0x43, 0x41];
pub(crate) const MAX_FILENAME_LENGTH: usize = 29;
