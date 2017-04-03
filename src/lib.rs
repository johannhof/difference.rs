//! Functions to find the difference between two texts (strings).
//! Usage
//! ----------
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! difference = "1.0"
//! ```
//!
//! Now you can use the crate in your code
//! ```ignore
//! extern crate difference;
//! ```
//!
//! ## Examples
//!
//! See [Examples.md](Examples.md) for more examples.
//!
//! ```rust
//! use difference::{Difference, Changeset};
//!
//! let changeset = Changeset::new("test", "tent", "");
//!
//! assert_eq!(changeset.diffs, vec![
//!   Difference::Same("te".to_string()),
//!   Difference::Rem("s".to_string()),
//!   Difference::Add("n".to_string()),
//!   Difference::Same("t".to_string())
//! ]);
//! ```

#![crate_name = "difference"]
#![doc(html_root_url = "http://docs.rs/difference")]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate term;

mod lcs;
mod merge;
mod display;

use lcs::lcs;
use merge::merge;
use std::io::prelude::*;

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

/// Struct to hold additional information about producing diff
pub struct ChangesetOptions {
    /// Display output of diff using words instead of colors
    /// # Example
    /// [-g-][+f+]oo
    pub word_diff: bool
}

impl ChangesetOptions {
    /// Returns a new ChangesetOptions with parameters
    pub fn new(word_diff: bool) -> ChangesetOptions {
        ChangesetOptions {
            word_diff: word_diff
        }
    }
}

/// The information about a full changeset
pub struct Changeset {
    /// An ordered vector of `Difference` objects, coresponding
    /// to the differences within the text
    pub diffs: Vec<Difference>,
    /// The split used when creating the `Changeset`
    /// Common splits are `""` for char-level, `" "` for word-level and `"\n"` for line-level.
    pub split: String,
    /// The edit distance of the `Changeset`
    pub distance: i32,
    /// Determines useage of words instead of color for diffs
    pub word_diff: bool
}

impl Changeset {
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
    /// use difference::{Changeset, Difference};
    ///
    /// let changeset = Changeset::new("test", "tent", "");
    ///
    /// assert_eq!(changeset.diffs, vec![
    ///     Difference::Same("te".to_string()),
    ///     Difference::Rem("s".to_string()),
    ///     Difference::Add("n".to_string()),
    ///     Difference::Same("t".to_string())
    /// ]);
    /// ```
    pub fn new(orig: &str, edit: &str, split: &str) -> Changeset {
        let (dist, common) = lcs(orig, edit, split);
        Changeset {
            diffs: merge(orig, edit, &common, split),
            split: split.to_string(),
            distance: dist,
            word_diff: false
        }
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
    /// This function allows a ChangesetOptions struct to be passed - tuning how the diffs are
    /// produced & displayed
    ///
    /// # Examples
    ///
    /// ```
    /// use difference::{Changeset, ChangesetOptions, Difference};
    /// 
    /// let changeset_options = ChangesetOptions::new(true);
    /// let changeset = Changeset::new_with_options("test", "tent", "", changeset_options);
    ///
    /// assert_eq!(changeset.diffs, vec![
    ///     Difference::Same("te".to_string()),
    ///     Difference::Rem("s".to_string()),
    ///     Difference::Add("n".to_string()),
    ///     Difference::Same("t".to_string())
    /// ]);
    /// ```
    pub fn new_with_options(orig: &str, edit: &str, split: &str, options: ChangesetOptions) -> Changeset {
        let (dist, common) = lcs(orig, edit, split);
        Changeset {
            diffs: merge(orig, edit, &common, split),
            split: split.to_string(),
            distance: dist,
            word_diff: options.word_diff
        }
    }
}

/// **This function is deprecated, please use `Changeset::new` instead**
///
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
#[deprecated(since="1.0.0", note="please use `Changeset::new` instead")]
pub fn diff(orig: &str, edit: &str, split: &str) -> (i32, Vec<Difference>) {
    let ch = Changeset::new(orig, edit, split);
    (ch.distance, ch.diffs)
}

/// Assert the difference between two strings. Works like diff, but takes
/// a fourth parameter that is the expected edit distance (e.g. 0 if you want to
/// test for equality).
///
/// To include this macro use:
///
/// ```
/// #[macro_use(assert_diff)]
/// extern crate difference;
/// # fn main() { }
/// ```
///
/// Remember that edit distance might not be equal to your understanding of difference,
/// for example the words "Rust" and "Dust" have an edit distance of 2 because two changes (a
/// removal and an addition) are required to make them look the same.
///
/// Will print an error with a colorful diff in case of failure.
#[macro_export]
macro_rules! assert_diff {
    ($orig:expr , $edit:expr, $split: expr, $expected: expr) => ({
        let orig = $orig;
        let edit = $edit;

        let changeset = $crate::Changeset::new(orig, edit, &($split));
        if changeset.distance != $expected {
            println!("{}", changeset);
            panic!("assertion failed: edit distance between {:?} and {:?} is {} and not {}, see \
                    diffset above",
                   orig,
                   edit,
                   changeset.distance,
                   &($expected))
        }
    })
}

/// Prints a colorful visual representation of the diff to standard out.
/// This is a convenience function for printing colored diff results.
/// 
/// The difference between this & the display impl is this uses the Term crate for colors,
/// allowing colors to appear in windows terminals
///
/// I recommend checking out the examples on how to build your
/// own diff output.
/// # Examples
///
/// ```
/// use difference::print_diff;
/// 
/// let changeset_options = difference::ChangesetOptions::new(false);
/// print_diff("Diffs are awesome", "Diffs are cool", " ", changeset_options);
/// ```
pub fn print_diff(orig: &str, edit: &str, split: &str, options: ChangesetOptions) -> Result<(), std::io::Error> {
    let ch = Changeset::new_with_options(orig, edit, split, options);
    let mut t = term::stdout().unwrap();

    for d in &ch.diffs {
        t.reset().unwrap();
        if ch.word_diff {
            match *d {
                Difference::Same(ref x) => try!(write!(t, "{}{}", x, ch.split)),
                Difference::Add(ref x) => try!(write!(t, "[-{}-]{}", x, ch.split)),
                Difference::Rem(ref x) => try!(write!(t, "[+{}+]{}", x, ch.split)),
            };
        } else {
            match *d {
                Difference::Same(ref x) => {
                    try!(write!(t, "{}{}", x, ch.split));
                },
                Difference::Add(ref x) => {
                    t.fg(term::color::GREEN).unwrap();
                    try!(write!(t, "{}{}", x, ch.split));
                },
                Difference::Rem(ref x) => {
                    t.fg(term::color::RED).unwrap();
                    try!(write!(t, "{}{}", x, ch.split));
                }
            };
        }
    }
    t.reset().unwrap();
    try!(writeln!(t, "")); // W/o this - terminals will print '%'
    Ok(())
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

    let changeset = Changeset::new(text1, text2, "\n");

    assert_eq!(changeset.distance, 4);

    assert_eq!(changeset.diffs,
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
