# bevy_template
This repository is a useful template for users that want to build bevy and need more than what a cargo-init can get you

# Useful tips

You can do offline programming by downloading the packages using this command;
```
cargo vendor
```
and add the following in your .cargo/Cargo.toml
```
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

