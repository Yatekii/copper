# Building

## Building for Linux

Make sure `libgtk3` and `libepoxy` are installed on your system.
Make sure you have the current nightly rustc enabled. You can do so with

```
rustup override set nightly
rustup update
```

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