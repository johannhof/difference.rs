//! Functions to find the difference between to texts (strings).

#![crate_name = "text_diff"]

#![feature(step_by)]

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
    Rem(String)
}

/// Calculates the edit distance and the changeset for two given strings
/// The first string is assumed to be the "original", the second to be an
/// edited version of the first
pub fn diff(orig: &str, edit: &str) -> (i32, Vec<Difference>) {
    let (dist, common) = lcs(orig, edit);
    (dist, merge(orig, edit, &common))
}

/// Generates a visual diffset
pub fn visual_diff(orig: &str, edit: &str) -> String {
    let (_, changeset) = diff(orig, edit);
    let mut ret = String::new();

    for seq in changeset {
        match seq {
            Difference::Same(ref x) => {
                ret.push_str(x);
            },
            Difference::Add(ref x) => {
                ret.push_str("\x1B[92m");
                ret.push_str(x);
                ret.push_str("\x1B[0m");
            },
            Difference::Rem(ref x) => {
                ret.push_str("\x1B[91m");
                ret.push_str(x);
                ret.push_str("\x1B[0m");
            }
        }
    }
    println!("{}", ret);

    ret
}

#[test]
fn test_visual_diff() {
    assert_eq!(visual_diff("test", "tost"), "t\x1B[92mo\x1B[0m\x1B[91me\x1B[0mst".to_string());
}
