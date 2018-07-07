# Copper

Copper is a PCB EDA Tool written in Rust.

# Motivation

Coming from an electrical engineering background, we have used quite a few PCB EDA tools.
Some of them are expensive and provide a lot of tools, that can work great or still be buggy.
Some of them are open source but don't really seem to make progress in terms of neither stability nor features.

We genuinely think that this is not a misery coming from incompetent developers, but from a really old code base that is just lackluster and rusty (ha ha).

Thus we decided to try and write our own tooling.

# Current state

Currently a basic [KiCad](http://kicad-pcb.org/) schema can be rendered. Text rendering is disabled for now as the development of [Pathfinder 2 by pcwalton](https://github.com/pcwalton/pathfinder) is awaited.

A basic UI with just some info, made with GTK3, is working.

# Roadmap

The idea is to complete a schema renderer first with regards to the PCB layouter in the future when it comes to performance questions.
To start the project off, the idea is to use the KiCad formats, such that KiCad as a tool can be replaced piece by piece.

1. Schema renderer
    - Text rendering on schema
    - Manipulation
        - Moving & rotating components
        - Adding & removing components
    - Electrical logic correctness
        - Tracking and validation of connections between components
            - With regards to netlist generation
    - General Info HUD
        - et al.
    - Export the schema back to the KiCad format
    - Store to a modern format (XML, JSON, YAML, TOML, RON, et al.) to maintain better readability, maintainability and serializing
    - Generate a Netlist in the KiCad format
    - (optional) Export to a modern format

2. Layouter
    - Basic rendering
    - more to come

# Project Structure

The project consist of
    
- a library which contains all the operations for parsing, rendering and manipulation. The API to manipulate the designs programmatically should be a first class citizen from the get go to enable tests to the very last layer before the GUI and future possibilities of scripting and a console. A few remarks:
    - The library is thematically structured into
        - `drawing` (everything involved in the process of drawing the designs)
        - `geometry` (for the moment contains various types; not sure if really need on the top level
        - `manipulation` (everything involved in the process of manipulating the designs such as placing, moving, rotating and editing components)
        - `parsing` (everything involved in the process of parsing the project design files)
        - `utils` (helpers to make the coders life easier)
    - For KiCad formats parsing, [nom](https://github.com/Geal/nom) is used. For a future modern format, [https://github.com/serde-rs/serde](serde) can be used.
    - No UI code should go in here.
- a binary, which renders the schemata. This binary should only contain GUI code + glue which interfaces the copper library to draw and manipulate the schemata.
    - For GUI [https://github.com/antoyo/relm](relm) is used, which is based on [https://github.com/gtk-rs/gtk](gtk-rs).
- future binaries will be added for different tasks.

# Building

## Building for Linux

Make sure `libgtk3` and `libepoxy` are installed on your system.
Make sure you have the current nightly rustc enabled. You can do so with

```
rustup override set nightly
rustup update
```

Inside the repository.
Finally you should be able to

```
cargo run --release test_data/kicad.lib test_data/kicad.sch
```

## Building for Windows

Set the active rustup toolchain using

```
rustup target add x86_64-pc-windows-gnu
rustup override set nightly-x86_64-pc-windows-gnu
rustup update
```

Install [MSYS2](http://www.msys2.org/).

In the “MSYS2 MSYS Shell” install libgtk3.

```
pacman -S mingw-w64-x86_64-gtk3
```

Then install the mingw toolchain

```
pacman -S mingw-w64-x86_64-toolchain
```

Start the “MSYS2 MinGW Shell” (not to be confused with “MSYS2 MSYS Shell”).
All future operations should be performed from this shell.

Add the rustup/cargo binaries to the `PATH`.
You can do this by adding the line

```
export PATH=$PATH:/c/Users/<your_name>/.cargo/bin:
```

to the `.bashrc`.
Restart the shell.

Now make sure the MYSYS2 environment is up to date (weirdly enough this is not the case after doing a fresh install). If you are prompted about conflicts between packages, just accept that pacman resolves the conflicts by uninstalling one.

```
pacman -Su
```

Restart the shell again (You might have to force close it, that's okay!)

```
pacman -Su
```

again. Now a ton of packages should get listed. Install them.

Now you should be able to

```
cargo run --release test_data/kicad.lib test_data/kicad.sch
```

# Contributing

If you would like to contribute code, ideas or criticism, please do not hesitate to do so.
We do not think that we already know how to perfectly write this application, so we are very open for big adjustments as well.

For now there is no "official process" of contruibuting code, so please just open issues and pull requests at will ;)

# Contact

Yatekii can be found on the mozilla and freenode IRC networks with the same nick, where he wanders in various rust and general programming related channels.
Sometimes tiwalun can be found there too if you get lucky.