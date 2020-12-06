# BetterThanVim
**BetterThanVim** (**BTV**) is an `ed`-inspired command-based command line text editor. Made for [queer_hack 2020](https://queer-hack.devpost.com/).

<img src="gayrust.jpg" width="200">

_(This image is [not made by me](https://twitter.com/whoisaldeka/status/1165148059484880896) and is licensed under CC-BY. Unfortunately there is not a bisexual version.)_

## Is BTV really better than Vim?
Yes.

Let me explain:
1. BTV, like `ed`, allows you to edit using an older interface like a printer or typewriter. [Yes, actually](https://www.youtube.com/watch?v=8vmOTvRXZ0E).
2. BTV is relatively lightweight, whereas Vim is highly complex and includes many unnecessary features like a built-in scripting language and syntax highlighting.
3. BTV has less commands/symbols to memorize.
4. BTV is written in Rust. You might be wondering: why does this make BTV better? Well, just take a look at [this crab](https://www.rustacean.net/).
5. BTV doesn't steal you data. Not saying that Vim does, I'm just saying that BTV doesn't.

## Commands (WIP)
First, BTV can be run (as of now) by doing `cargo run -- file.md` where `file.md` is the file you want to edit (if the file does not exist, then BTV will create an empty file). Then, BTV will ask for commands.

Secondly, some terminology:
* The _current address_ is a specific line number that BTV is pointing at. This is similar to a cursor in an interactive text editor. BTV will start with the current address pointed to the last line of the file. Note that the line numbers (for now) start at 0, as is customary in programming.
* The _buffer_ is the modified contents of your file stored in memory. Keep in mind, the buffer will not be saved to disk until you, well, save it to disk (you can do this using `w` as noted below).

| Command | Description
| :------ | :------
| `.` | Prints the line at the current address.
| `.<n>`, where `<n>` is a number | Prints the line at the given address.
| `r` | Prints the current address.
| `^` | Set the current address to the first line.
| `$` | Set the current address to the last line.
| `+` | Increment the current address by one line.
| `-` | Decrement the current address by one line.
| `<n>`, where `<n>` is a number | Sets the current address to the line at the given `<n>`.
| `a` | Enters insert mode, appending after the current address. Inputting `.` on a new line will exit insert mode. Note that with each line inserted, the current address will increment.
| `A` | Enters insert mode, prepending before the current address. Inputting `.` on a new line will exit insert mode. Again, with each line inserted, the current address will increment.
| `c` | Enters insert mode, replacing the line pointed to by the current address. Inputting `.` will exit insert mode without changes.
| `d` | Deletes the line pointed to by the current address. If this at the end of the file, then this will decrement the current address.
| `y` | Yanks (copies) the line pointed to by the current address. This will put the line in an internal buffer rather than the clipboard.
| `x` | Cuts the line pointed to by the current address, deleting but placing it in the internal buffer.
| `p` | Pastes whatever was last yanked after the current address, assuming that the internal yank buffer isn't empty.
| `w` | Saves the buffer to disk.
| `q` | Quits BTV.

## Technologies used
* [Rust](https://github.com/rust-lang/rust)
  * [Cargo](https://github.com/rust-lang/cargo)
  * [structopt](https://github.com/TeXitoi/structopt)
  * [regex](https://github.com/rust-lang/regex)

## Authors
1. Eric Schneider
