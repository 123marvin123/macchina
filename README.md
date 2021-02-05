<h1 align="center"> Macchina </h1>

<p align="center">
  <img src="preview.png"/>
  <a href="https://forthebadge.com/images/badges/made-with-rust.svg" alt="Made With Rust Badge"></a>
</p>

![madewith](https://img.shields.io/static/v1?label=MADE%20WITH&message=RUST&color=ef4041&style=for-the-badge)
![reposize](https://img.shields.io/github/repo-size/grtcdr/macchina?color=BEE5BF&logo=github&style=for-the-badge)
![loc](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=FFD1BA&label=Lines%20of%20Code&logo=rust&style=for-the-badge)

## Table of Contents:
- [About](#about)
- [Changelog](#change)
- [Benchmarks](#bench)
- [Features](#features)
- [Installation](#install)
- [Platform Support](#platform-support)

---

## About Macchina <a name="about"></a>
Macchina lets you view basic system information, like your hostname, your kernel version, memory usage, and much more.
No one wants a slow fetcher, and macchina's main goal is to provide you with handy features while keeping performance a priority.

---

## Changelog <a name="change"></a>
- You can __visualize__ memory usage and battery percentage using progress bars

---

## Benchmarks <a name="bench"></a>
Macchina is pretty fast, see for yourself:

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 22.2 ± 0.7 | 21.0 | 25.1 | 1.00 |
| `neofetch` | 243.9 ± 2.3 | 240.0 | 246.9 | 11.01 ± 0.37 |

__Summary__: `macchina` runs __11.01 ± 0.37__ times __faster__ than `neofetch`

- Note that hiding elements using Macchina's __--hide__ argument significantly improves speed

---

## Features <a name="features"></a>
Macchina displays basic system information such as:
- Hostname
- Operating system
- Kernel version
- Package count _(Arch-based distributions only, will print __0__ on any other distribution)_
- Shell path/name in which macchina was ran
- Terminal instance name in which macchina was ran
- Processor _model name_, _frequency_ and _thread count_
- Uptime
- Memory usage
- Battery _percentage_ and _status_
- Palette (if user runs Macchina with `--palette / -p`)

Macchina supports the following arguments:
- `--no-color` -> disable colors
- `--color <color>` -> specify the key color
- `--separator-color <color>` -> specify the separator color
- `--random-color` -> let macchina choose a random color for you
- `--palette` -> display palette
- `--short-sh` -> shorten shell output (/bin/zsh => zsh)
- `--hide <element>` -> hide elements such as host, os, kern, etc.
- `--bar` -> display memory usage and battery percentage as progress bars
- `--theme <theme_name>` -> change themes
- `--help` -> display help menu
- `--version` -> print version
- `--padding <amount>` -> specify the amount of padding to use

---

## Installation <a name="install"></a>

Macchina is not yet ready to be deployed on [crates.io](https://crates.io/), but you can compile it from source and play around with it.

Here's how _you_ can do that:

1. Clone the repo: `git clone https://github.com/grtcdr/macchina`
2. Navigate to the folder and compile from source: `cd macchina && cargo build`
3. __target/__ has been generated by cargo and Macchina's binary file can now be run: `./target/debug/macchina`

__Bonus__: To run macchina from anywhere on your system, you have two options:

1. Place `macchina/target/debug/macchina` somewhere in your __$PATH__, like _~/.local/bin_ or _/usr/bin_.

:heavy_exclamation_mark: Any changes you make to the source code will apply to the macchina binary file but you'll need to place the newly built binary file on your __$PATH__ __again__ to run it from _anywhere on your system_ with your new changes.

2. Create a new symlink for Macchina:

:heavy_exclamation_mark: This symlink will point to the binary file, so everytime you modify the source code and rebuild, running `$ macchina` from _anywhere on your system_ will run the newly built binary file.

---

## Will Macchina Work on Your Macchina? <a name="platform-support"></a>

|  Platform     |      Support       |
| :-:           |        :-:         |
| Linux         | :heavy_check_mark: |
| BSD           |     :question:     |
| MacOS         |                    |
| Windows       |                    |

> Cells containing :question: have not yet been tested.
