# crate yew-bootstrap-icons

<!-- cargo-rdme start -->

Bootstrap icons for yew

## Version

This release is for yew 0.2.0 and contains bootstrap-icons-v1.10.5.

## Icons

All icons are available as a constant:
```rust
let icon = BI::HEART;
let empty_icon = BI::empty(); // or BI::default()
html!{
    <h1>{"I"} {icon} {BI::GEAR}</h1>
}
```

## Required CSS

Using this crate, and Bootstrap icons, requires the inclusion of a CSS file from Bootstrap (`bootstrap-icons.css`). This file can be added by:

* Copying it yourself from the Bootstrap website
* Using `BIFiles::copy()` in build-executed code (see below)
* Using `BIFiles::cdn()` (easiest)
* Accessing the data via `BIFiles::FILES` and deliver them yourself

## Installation - Quickstart - Using CDN

0. Call `BIFiles::cdn()` inside the `html!{}` returned by your application

## Installation - Automatically Copying Files

This is not required if you take the `BIFiles::cdn()` or `BIFiles::FILES` approach.

0. Specify your wasm-program in `index.html`:
   ```html
   <link data-trunk rel="rust" data-bin="name-of-app" />
   ```
   (since we'll be writing a build-executed program, there are now two binaries and trunk needs to know which is your WASM binary)

1. Add the build-executed binary to your `Cargo.toml`
   ```toml
   [[bin]]
   name = "copy-bootstrap-icons"
   ```

### Option 1: Copy to dist

2. Create the file `src/bin/copy-bootstrap-icons.rs` with:
   ```rust
   use std::path::PathBuf;
   use yew_bootstrap_icons::BIFiles;

   fn main() -> Result<(), std::io::Error> {
       let path = PathBuf::from(
           std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
       )
       .join(BIFiles::NAME);
       if !path.is_dir() {
           std::fs::create_dir(&path)?;
       }
       BIFiles::copy(&path)
   }
   ```

3. Add the css to your `index.html`
   ```html
   <link rel="stylesheet" href="bootstrap-icons-v1.10.5/bootstrap-icons.css" />
   ```
   (Don't forget to set `<base data-trunk-public-url />`.)

4. Add the program to your `Trunk.toml`
   ```toml
   [[hooks]]
   stage = "build"
   command = "cargo"
   command_arguments = ["run", "--bin", "copy-bootstrap-icons"]
   ```

### Option 2: Copy to source (and let trunk copy it to dist)

This means that trunk will add the hash to the css-file.

It is assumed that your directory for static files is called `static`, if not
change the paths below.

2. Create the file `src/bin/copy-bootstrap-icons.rs` with:
   ```rust
   use std::path::PathBuf;
   use yew_bootstrap_icons::BIFiles;

   fn main() -> Result<(), std::io::Error> {
       let path = &PathBuf::from(
           std::env::var("TRUNK_SOURCE_DIR").expect("Environment variable TRUNK_SOURCE_DIR"),
       )
       .join("static")
       .join(BIFiles::NAME);
       if !path.is_dir() {
           std::fs::create_dir(&path)?;
       }
       BIFiles::copy(&path)
   }
   ```

3. Add the css to your `index.html`
   ```html
   <link data-trunk rel="css" href="static/bootstrap-icons-v1.10.5/bootstrap-icons.css" />
   <link data-trunk rel="copy-dir" href="static/bootstrap-icons-v1.10.5/fonts" />
   ```

4. Add the program to your `Trunk.toml`
   ```toml
   [[hooks]]
   stage = "pre_build"
   command = "cargo"
   command_arguments = ["run", "--bin", "copy-bootstrap-icons"]
   ```

<!-- cargo-rdme end -->
## License

This project is licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](https://github.com/alexkazik/ownable/blob/main/LICENSE-MIT))
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](https://github.com/alexkazik/ownable/blob/main/LICENSE-APACHE))

at your option.
