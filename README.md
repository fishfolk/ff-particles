# ff-particles

This is a fork of [macroquad-particles](https://github.com/not-fl3/macroquad/tree/master/particles) that is a result of our requirements for [Fish Fight](https://github.com/fishfight/FishFight).

The original plan was to just extend it with serde support, but we have decided to extend it further, as required by the [FishFight](https://github.com/fishfight/FishFight) project.

## Usage

To get started, add the following to your `Cargo.toml`:

```toml
[dependencies]
ff-particles = "0.1"
```

### Crate features

- `nanoserde` enable support for nanoserde
- `serde` enable support for serde

## License

Macroquad and the original code is Copyright (c) 2020-2021 Fedor Logachev

The project is dual licensed (MIT/Apache-2.0)

