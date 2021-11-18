# ff-particles

This is a fork of [macroquad-particles](https://github.com/not-fl3/macroquad/tree/master/particles) that is a result of our requirements for [Fish Fight](https://github.com/fishfight/FishFight). Initially, we just wanted to add serde support, but we have decided to extend it further in the near future. 

## Usage

To get started, add the following to your `Cargo.toml`:

```toml
[dependencies]
ff-particles = "0.1"
```

### Crate features

- `nanoserde` enable support for nanoserde
- `serde` enable support for serde