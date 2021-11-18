# macroquad-particles-serde

This is a fork of [macroquad-particles](https://github.com/not-fl3/macroquad/tree/master/particles) that adds serde support

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
macroquad-particles-serde = "0.1"
```

Serde support is enabled as a default feature. If you, for some reason, want to disable it, add the following, in stead:

```toml
[dependencies]
macroquad-particles-serde = { version = "0.1", default-features = false }
```
