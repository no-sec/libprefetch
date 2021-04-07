# libprefetch

[![Crates.io](https://img.shields.io/crates/v/libprefetch.svg)](https://crates.io/crates/libprefetch)
[![Crates.io](https://img.shields.io/crates/d/libprefetch.svg)](https://crates.io/crates/libprefetch)
[![license](http://img.shields.io/badge/license-WTFPL-blue.svg)](https://github.com/zadlg/libprefetch/blob/master/LICENSE)

A forensic library which parses and reads Microsoft Prefetch files.

`libprefetch` fully supports the following versions of Windows:
  * Windows 2003
  * Windows XP
  * Windows Vista
  * Windows 7
  * Windows 8/8.1

`libprefetch` **partially supports** Windows 10.

Features:
  * Parser and validator
  * Auto detects version of Windows
  * Provides the last execution time and the execution counter
  * Provides metric information about loaded files (like dll etc) **if available**, such as :
    * filename
    * start time
    * duration
    * average duration
    * NTFS MFT entry
    * NTFS sequence numer
  * Provides the trace chains (**unavailable for Windows 10**)
  * Provides all pieces of information about the volumes:
    * device path
    * creation time
    * serial number
    * list of directories

This library will be used in a global forensic computing library very soon.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libprefetch = "0.1.1"
```

and this to your crate root:

```rust
extern crate libprefetch;
```

## Example

```rust
use libprefetch::Prefetch;

let file = std::fs::File::open("assets/WUAUCLT.EXE-399A8E72.pf").unwrap();

let prefetch = Prefetch::new(file).unwrap();

// Prints some information
println!("Executable {} launched {} times. The last time was: {}",
  prefetch.name(),
  prefetch.execution_counter(),
  prefetch.last_execution_time() // TODO: format the FILETIME here
);

// Iterates over all loaded DLL etc for the prefetch file
println!(" ===== File metrics ===== ");
for metric in prefetch.metrics().unwrap() {
  println!("#{}: {}", metric.id(), metric.filename());
  println!("    start time: {}", metric.start_time().unwrap());
  println!("    duration: {}", metric.duration().unwrap());
  println!(" ------------------------------- ");
}

// Iterates over the volumes
println!(" ===== Volumes ===== ");
for volume in prefetch.volumes().unwrap() {
  println!("Volume #{}:", volume.id());
  println!("    Path: {}", volume.device_path());
  println!("    Creation time: {}", volume.creation_time());
  println!("    Serial number: {}", volume.serial_number());
  println!("    Directories: ");
  for directory in volume.directories().unwrap() {
    println!("        {}", directory);
  }
}


```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## Compatibility

`libprefetch` seems to work for rust 1.9 and greater.

## License

<http://www.wtfpl.net/about/>
