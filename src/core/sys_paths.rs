use once_cell::sync::Lazy;
use std::fs;

// The purpose of this module is to perform a single read of heavily-accessed
// sys paths (like /proc/cpuinfo or /proc/meminfo) on the very first access.
// Since rayon executes modules entirely in parallel, multiple threads hitting the
// exact same /proc read simultaneously generates expensive overlapping syscalls.
// By delegating these to Lazy<String>, the first thread reads the file, and
// all other parallel modules retrieve the String from memory instantly without overhead.

pub static CPUINFO: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("/proc/cpuinfo").unwrap_or_default()
});

pub static MEMINFO: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("/proc/meminfo").unwrap_or_default()
});

pub static OS_RELEASE: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("/etc/os-release").unwrap_or_else(|_| {
        fs::read_to_string("/usr/lib/os-release").unwrap_or_default()
    })
});
