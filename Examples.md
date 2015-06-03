% Diff Examples

These examples use the official term library to format output.

## Line-by-line (Git Style)

![](https://raw.githubusercontent.com/johannhof/text-diff.rs/master/assets/git-style.png)

The only thing to do here is to create a diff based on line splits (passing the newline character as a split symbol)
and iterate over the results, matching and formatting them based on the type of `Difference`.

```rust
extern crate term;
extern crate text_diff;
use std::io::Write;
use text_diff::Difference;

fn main() {
  let text1 = "Roses are red, violets are blue,\n\
               I wrote this library here,\n\
               just for you.\n\
               (It's true).";

  let text2 = "Roses are red, violets are blue,\n\
               I wrote this documentation here,\n\
               just for you.\n\
               (It's quite true).";

  // compare both texts, the third parameter defines the split level
  let (_dist, changeset) = text_diff::diff(text1, text2, "\n");

  let mut t = term::stdout().unwrap();

  for i in 0..changeset.len() {
      match changeset[i] {
          Difference::Same(ref x) => {
              t.reset().unwrap();
              writeln!(t, " {}", x);
          },
          Difference::Add(ref x) => {
              t.fg(term::color::GREEN).unwrap();
              writeln!(t, "+{}", x);
          },
          Difference::Rem(ref x) => {
              t.fg(term::color::RED).unwrap();
              writeln!(t, "-{}", x);
          }
      }
  }
  t.reset().unwrap();
  t.flush().unwrap();

}

```

## Underline words (GitHub Style)

![](https://raw.githubusercontent.com/johannhof/text-diff.rs/master/assets/word-underline.png)

```rust
extern crate term;
extern crate text_diff;
use std::io::Write;
use text_diff::Difference;

fn main() {

  let text1 = "Roses are red, violets are blue.";
  let text2 = "Roses are blue, violets are";

  let mut t = term::stdout().unwrap();

  let (_, changes) = text_diff::diff(text1, text2, "");

  for c in changes.iter() {
    match c {
      &Difference::Same(ref z) => {
        t.fg(term::color::RED).unwrap();
        write!(t, "{}", z);
      },
      &Difference::Rem(ref z) => {
        t.fg(term::color::WHITE).unwrap();
        t.bg(term::color::RED).unwrap();
        write!(t, "{}", z);
        t.reset().unwrap();
      },
      _ => ()
    }
  }
  t.reset().unwrap();

  writeln!(t, "");

  for c in changes.iter() {
    match c {
      &Difference::Same(ref z) => {
        t.fg(term::color::GREEN).unwrap();
        write!(t, "{}", z);
      },
      &Difference::Add(ref z) => {
        t.fg(term::color::WHITE).unwrap();
        t.bg(term::color::GREEN).unwrap();
        write!(t, "{}", z);
        t.reset().unwrap();
      },
      _ => ()
    }
  }
  t.reset().unwrap();
  t.flush().unwrap();

  assert!(false);
}
```

## GitHub Style (Multiline)

![](https://raw.githubusercontent.com/johannhof/text-diff.rs/master/assets/github-style.png)

```rust
extern crate term;
extern crate text_diff;
use std::io::Write;
use text_diff::Difference;

fn main() {
  let text1 = "Roses are red, violets are blue,\n\
               I wrote this library here,\n\
               just for you.\n\
               (It's true).";

  let text2 = "Roses are red, violets are blue,\n\
               I wrote this documentation here,\n\
               just for you.\n\
               (It's quite true).";

  let (dist, changeset) = text_diff::diff(text1, text2, "\n");

  let mut t = term::stdout().unwrap();

  for i in 0..changeset.len() {
      match changeset[i] {
          Difference::Same(ref x) => {
              t.reset().unwrap();
              writeln!(t, " {}", x);
          },
          Difference::Add(ref x) => {
              match changeset[i - 1] {
                Difference::Rem(ref y) => {
                  t.fg(term::color::GREEN).unwrap();
                  write!(t, "+");
                  let (_, changes) = text_diff::diff(y, x, " ");
                  for c in changes {
                    match c {
                      Difference::Same(ref z) => {
                        t.fg(term::color::GREEN).unwrap();
                        write!(t, "{}", z);
                        write!(t, " ");
                      },
                      Difference::Add(ref z) => {
                        t.fg(term::color::WHITE).unwrap();
                        t.bg(term::color::GREEN).unwrap();
                        write!(t, "{}", z);
                        t.reset().unwrap();
                        write!(t, " ");
                      },
                      _ => ()
                    }
                  }
                  writeln!(t, "");
                },
                _ => {
                  t.fg(term::color::BRIGHT_GREEN).unwrap();
                  writeln!(t, "+{}", x);
                }
              };
          },
          Difference::Rem(ref x) => {
              t.fg(term::color::RED).unwrap();
              writeln!(t, "-{}", x);
          }
      }
  }
  t.reset().unwrap();
  t.flush().unwrap();
}

```
