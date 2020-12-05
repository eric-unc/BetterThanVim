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
First, BTV can be run (as of now) by doing `cargo run -- file.md` where `file.md` is the file you want to edit. Then, BTV will ask for commands.

Secondly, some terminology: the _current address_ is a specific line number that BTV is pointing at. This is similar to a cursor in an interactive text editor. The _buffer_ is the modified contents of your file stored in memory. Keep in mind, the buffer will not be saved to disk until you, well, save it to disk (you can do this using `w` as noted below).

| Command | Description
| :------ | :------
| `.` | Prints the line at the current address.
| `a` | Enters append mode, appending after the current address. Inputting `.` on a new line will exit append mode.
| `w` | Saves the buffer to disk.
| `q` | Quits BTV.

## Technologies used
1. [Rust](https://github.com/rust-lang/rust) (with [Cargo](https://github.com/rust-lang/cargo))
2. [ed](https://www.gnu.org/software/ed/)
3. [Vim](https://github.com/vim/vim) (unfortunately)
4. BetterThanVim (fortunately)

## Authors
1. Eric Schneider

