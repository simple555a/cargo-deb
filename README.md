This cargo subcommand will largely automate the process of building a Debian package. In order to get a Rust project build with `cargo deb`, you must add a [packages.metadata.deb] table to your Cargo.toml file. You must also ensure that you have filled out the minimal package information, particularly the `description` and `repository` values.

### Example [package.metadata.deb]
The required keys are `maintainer`, `copyright`, `license_file`, `depends`, `extended_description`, `section`, `priority`, and `assets`.

The `license_file` parameter contains the location of the license file followed by the number of lines to skip (because Debian uses it's own copyright format).

The `assets` are a list of files that will be installed into the system.
- The first argument of each asset is the location of that asset in the Rust project.
- The second argument is where the file will be copied.
    - If is argument ends with **/** it will be inferred that the target is the directory where the file will be copied.
    - Otherwise, it will be inferred that the source argument will be renamed when copied.
- The third argument is the permissions to assign that file via chmod.

### Running `cargo deb`
Upon running `cargo deb` from the base directory of your Rust project, a Debian package will be saved in the same
directory. If you would like to handle the build process yourself, you can use `cargo deb --no-build` so that the
`cargo-deb` command will not attempt to rebuild your project.

# Cargo Deb Example

```toml
[package.metadata.deb]
maintainer = "Michael Aaron Murphy <mmstickman@gmail.com>"
copyright = "2016, Michael Aaron Murphy <mmstickman@gmail.com>"
license_file = ["LICENSE", "4"]
extended_description = """\
A simple subcommand for the Cargo package manager for \
building Debian packages from Rust projects."""
depends = "libc6"
section = "utility"
priority = "optional"
assets = [
    ["target/release/cargo-deb", "usr/bin/", "755"],
    ["README.md", "usr/share/doc/cargo-deb/README", "644"],
]
```

# Systemd Manager Example

```toml
[package.metadata.deb]
maintainer = "Michael Aaron Murphy <mmstickman@gmail.com>"
copyright = "2015-2016, Michael Aaron Murphy <mmstickman@gmail.com>"
license_file = ["LICENSE", "3"]
depends = "libc6, libgtk-3-0 (>= 3.16)"
extended_description = """\
Written safely in Rust, this systemd manager provides a simple GTK3 GUI interface \
that allows you to enable/disable/start/stop services, monitor service logs, and \
edit unit files without ever using the terminal."""
section = "admin"
priority = "optional"
assets = [
    ["assets/org.freedesktop.policykit.systemd-manager.policy", "usr/share/polkit-1/actions/", "644"],
    ["assets/systemd-manager.desktop", "usr/share/applications/", "644"],
    ["assets/systemd-manager-pkexec", "usr/bin/", "755"],
    ["target/release/systemd-manager", "usr/bin/", "755"]
]
```
