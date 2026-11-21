// This file is part of the uutils util-linux package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use uutests::{at_and_ucmd, new_ucmd};

#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails().code_is(1);
}

#[test]
fn test_missing_args() {
    new_ucmd!()
        .fails()
        .code_is(1)
        .stderr_contains("required arguments were not provided");
}

#[cfg(target_os = "linux")]
#[test]
fn test_generate_destination() {
    let (at, mut ucmd) = at_and_ucmd!();
    at.write("q", "0123456789");
    ucmd.args(&["q", "z", "2:0:3"]).succeeds();
    assert_eq!(at.read("z"), "234");
}

#[cfg(target_os = "linux")]
#[test]
fn test_multiple_ranges_accepted() {
    let (at, mut ucmd) = at_and_ucmd!();
    at.write("q", "0123456789");
    ucmd.args(&["q", "z", "0::5", "1::"]).succeeds();
}
