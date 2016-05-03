//! Functions to find the difference between two texts (strings).
//! Usage
//! ----------
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! difference = "0.4"
//! ```
//!
//! Now you can use the crate in your code
//!
//! ```ignore
//! extern crate difference;
//! ```

#![crate_name = "difference"]
#![doc(html_root_url = "https://johannhof.github.io/difference.rs/")]

// I can basically feel the karma already
#![deny(missing_docs)]

mod lcs;
mod merge;

use lcs::lcs;
use merge::merge;

/// Defines the contents of a changeset
/// Changesets will be delivered in order of appearance in the original string
/// Sequences of the same kind will be grouped into one Difference
#[derive(PartialEq, Debug)]
pub enum Difference {
    /// Sequences that are the same
    Same(String),
    /// Sequences that are an addition (don't appear in the first string)
    Add(String),
    /// Sequences that are a removal (don't appear in the second string)
    Rem(String),
}

/// Calculates the edit distance and the changeset for two given strings.
/// The first string is assumed to be the "original", the second to be an
/// edited version of the first. The third parameter specifies how to split
/// the input strings, leading to a more or less exact comparison.
///
/// Common splits are `""` for char-level, `" "` for word-level and `"\n"` for line-level.
///
/// Outputs the edit distance (how much the two strings differ) and a "changeset", that is
/// a `Vec` containing `Difference`s.
///
/// # Examples
///
/// ```
/// use difference::diff;
/// use difference::Difference;
///
/// let (dist, changeset) = diff("test", "tent", "");
///
/// assert_eq!(changeset, vec![
///     Difference::Same("te".to_string()),
///     Difference::Rem("s".to_string()),
///     Difference::Add("n".to_string()),
///     Difference::Same("t".to_string())
/// ]);
/// ```
pub fn diff(orig: &str, edit: &str, split: &str) -> (i32, Vec<Difference>) {
    let (dist, common) = lcs(orig, edit, split);
    (dist, merge(orig, edit, &common, split))
}

/// Assert the difference between two strings. Works like diff, but takes
/// a fourth parameter that is the expected edit distance (e.g. 0 if you want to
/// test for equality).
///
/// Remember that edit distance might not be equal to your understanding of difference,
/// for example the words "Rust" and "Dust" have an edit distance of 2 because two changes (a
/// removal and an addition) are required to make them look the same.
///
/// Will print an error with a colorful diff using `print_diff` in case of failure.
#[macro_export]
macro_rules! assert_diff {
    ($orig:expr , $edit:expr, $split: expr, $expected: expr) => ({
        let orig = $orig;
        let edit = $edit;

        let (d, _) = $crate::diff(orig, edit, &($split));
        if d != $expected {
            $crate::print_diff(orig, edit, &($split));
            panic!("assertion failed: edit distance between {:?} and {:?} is {} and not {}, see \
                    diffset above",
                   orig,
                   edit,
                   d,
                   &($expected))
        }
    })
}

/// Prints a colorful visual representation of the diff.
/// This is just a convenience function for those who want quick results.
///
/// I recommend checking out the examples on how to build your
/// own diff output.
/// # Examples
///
/// ```
/// use difference::print_diff;
/// print_diff("Diffs are awesome", "Diffs are cool", " ");
/// ```
pub fn print_diff(orig: &str, edit: &str, split: &str) {
    let (_, changeset) = diff(orig, edit, split);
    let mut ret = String::new();

    for seq in changeset {
        match seq {
            Difference::Same(ref x) => {
                ret.push_str(x);
                ret.push_str(split);
            }
            Difference::Add(ref x) => {
                ret.push_str("\x1B[92m");
                ret.push_str(x);
                ret.push_str("\x1B[0m");
                ret.push_str(split);
            }
            Difference::Rem(ref x) => {
                ret.push_str("\x1B[91m");
                ret.push_str(x);
                ret.push_str("\x1B[0m");
                ret.push_str(split);
            }
        }
    }
    println!("{}", ret);
}

#[test]
fn test_diff() {
    let text1 = "Roses are red, violets are blue,\n\
                 I wrote this library,\n\
                 just for you.\n\
                 (It's true).";

    let text2 = "Roses are red, violets are blue,\n\
                 I wrote this documentation,\n\
                 just for you.\n\
                 (It's quite true).";

    let (dist, changeset) = diff(text1, text2, "\n");

    assert_eq!(dist, 4);

    assert_eq!(changeset,
               vec![Difference::Same("Roses are red, violets are blue,".to_string()),
                    Difference::Rem("I wrote this library,".to_string()),
                    Difference::Add("I wrote this documentation,".to_string()),
                    Difference::Same("just for you.".to_string()),
                    Difference::Rem("(It's true).".to_string()),
                    Difference::Add("(It's quite true).".to_string())]);
}

#[test]
#[should_panic]
fn test_assert_diff_panic() {
    let text1 = "Roses are red, violets are blue,\n\
                 I wrote this library,\n\
                 just for you.\n\
                 (It's true).";

    let text2 = "Roses are red, violets are blue,\n\
                 I wrote this documentation,\n\
                 just for you.\n\
                 (It's quite true).";

    assert_diff!(text1, text2, "\n'", 0);
}

#[test]
fn test_assert_diff() {
    let text1 = "Roses are red, violets are blue";

    let text2 = "Roses are green, violets are blue";

    assert_diff!(text1, text2, " ", 2);
}
