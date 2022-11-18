# PcSpecs

## Get system info

\
PcSpecs is a crate to gather the system specs.


## Features
- Hostname
- Platform
- OsNumber
- CPU
- GPU
- RAM
- Disk
- MAINBOARD

## Installation

Install the dependencies.

```sh
pcspecs = "0.1.0"
```

## How to use

Add imports:

```rust
use pcspecs::specs;
```

Example:

```rust



fn main(){
    println!("{:#?}", specs());
    /* Output
    DESKTOP-5DH6S74 
    Microsoft Windows 10
    Pro 10 
    Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz
    NVIDIA GeForce RTX 2060 SUPER
    24 
    134
    TUF GAMING B460M-PLUS (WI-FI*/
}
```

Or

```rust

fn main(){
    println!("{:#?}", specs().GPU);
    /* Output
    NVIDIA GeForce RTX 2060 SUPER
    */
}
```

## License

MIT

PcSpecs is [MIT licensed](LICENSE).