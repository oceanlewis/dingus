# Dingus

Dingus is a simple tool by and for the folks at Assemble to ease management of environment variables. Dingus supports two ways of applying environment variables (through the `print` and `session` subcommands), whichever you'd prefer is up to you.

### Requirements

Dingus is written in the Rust Programming Language and requires the user to have at least the Stable Rust toolchain installed to compile. The preferred way to manage Rust toolchains is via [rustup](https://www.rustup.rs). You can also just install Rust from your favorite package manage if that's more your style.

##### Rustup Installation
`curl https://sh.rustup.rs -sSf | sh`

##### Homebrew Installation
`brew install rust`

##### Debian Installation
As root: `apt-get install rustc`

Do note that your `$PATH` might need to be altered so that your system knows about `rustc` the Rust compiler, `cargo` Rust's package manager, and `~/.cargo/bin` the default location for compiled binaries to be installed. `rustup` will inform you of how to make the change during its installation process, your systems package manage may or may not be as benevolent.

### Keeping Dingus Up to Date

Once you have the Rust compiler installed you're free to install Dingus with `cargo install dingus`. I'd also recommend installing [cargo-update](https://github.com/nabijaczleweli/cargo-update) so you can update all your Rust binaries in one go.

`cargo install cargo-update` will download, compile and install the `cargo` subcommand `install-update`. Once installed you should be able to rust `cargo install-update -a` to update any binaries you've installed with `cargo` in a hands-free manner.

### Using Dingus

Dingus has some nice built in help messages in case you forget, but here's a quick tutorial regardless.

This file should exist at `~/.config/dingus/example_1.yaml` with the following contents:

```yaml
HELLO: Hello World!
MULTI_LINE: "Hello there,
How are you?"
```

This file should exist at `~/.config/dingus/example_2.yaml` with the following contents:

```yaml
HELLO: Hello, Dingus Session!
```

#### Dingus Print Example

Run `dingus print -c example_1.yaml`. See how `dingus` found the `example.yaml` file we created, read its contents, and printed out a command? That command can be piped into `eval` to set those variables directly in your current shell session. Neat, huh? `dingus` knows what shell you're running by looking at your `$SHELL` variable and printing out a command for that shell's syntax. I've only tested this in the `fish` and `bash` shells, so I don't know if I've got it right for all shells (actually, I know I haven't). If this doesn't work properly for you let me know what sytax I should be using for your shell and I'll toss it in there.

The full command to apply the variables to your shell is `dingus print -c example_1.yaml | eval`. Normally it's discouraged to pipe anything into `eval` since it can open up remote code execution vulnerabilities, but you're not doing this on a production server so it's cool (right?).

Check it out: `echo $HELLO` and `echo $MULTI_LINE` both contain the values you set in the file `~/.config/dingus/example_1.yaml`.

#### Dingus Session Example

In case you don't want to pollute your current shell session with environment variables, `dingus` also supports opening a new session for you. By default, `dingus` will use whatever command your `$SHELL` variable refers to and assume that's a valid shell to place you into.

Try running `dingus session -c example_2.yaml`. You're now in a new shell session. Try `echo $HELLO`. Yep, we've applied the variables from `~/.config/dingus/example_2.yaml`, which are now all accessible. Also available are any variables you set before entering the Dingus session, so if just ran the example in our "Dingus Print Example" section you'll find that `$MULTI_LINE` is still available.

Hope this explains stuff a bit and helps you save yourself some time!
