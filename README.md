# text-diff.rs [![](https://travis-ci.org/johannhof/text-diff.rs.svg?branch=master)](https://travis-ci.org/johannhof/text-diff.rs)
A Rust text diffing library. 

__[Documentation](https://johannhof.github.io/text-diff.rs)__

__[Examples](/Examples.md)__

![](https://raw.githubusercontent.com/johannhof/text-diff.rs/master/assets/fox.png)
![](https://raw.githubusercontent.com/johannhof/text-diff.rs/master/assets/github-style.png)

Usage
----------

Add the following to your Cargo.toml:

```toml
[dependencies.text_diff]

git = "https://github.com/johannhof/text-diff.rs.git"

```

Now you can use the crate in your code
```rust
extern crate text_diff;
```

Using the binary
-----------------

text-diff can also be used as a command-line application. The best way to install it is using rustle:

```
curl -sf https://raw.githubusercontent.com/brson/rustle/master/rustle.sh | sh -s -- https://github.com/johannhof/text-diff.rs
```
