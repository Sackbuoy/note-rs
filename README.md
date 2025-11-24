# note-rs

Just a script for note taking that I wrote as a simple rust project bc I wanted to, and I use vim for everything else so it just made sense.

also, since I know you'll forget, to set up zsh completions:
1. Add a `completions` dir somewhere to be in the list in the `$fpath` variable
you can persist it by adding the following to your ~/.zshrc
```
fpath=(~/newdir $fpath)
```
2. Enable completions on your machine by adding this to your ~/.zshrc.
This must happen _after_ all fpath edits in order to have an effect
```
autoload -U compinit
compinit
```
3. Then in that directory put a file called `_note`(or whatever the bin is prefixed with a \_) and put this in it:
```
#compdef _note note

function _note() {
  _arguments '1: :_path_files -W /path/to/notes/dir'
}
```
4. reload your shell and it should work

itd be cool if i could note hardcode that path lol but the config directory won't always be the same so i cant always pull from there. maybe and env var or something. great problem for future me

# building with nix:
1. `nix build` -> consults `default.nix` to build the cargo package
    - this creates the binary in `./result/bin/note`
2. `nix develop` -> consults `shell.nix` to create a dev environment
3. `nix run` -> runs `cargo run`, you can add args like so `nix run . -- arg1 arg2`

# example configuration file:
```
# ~/.note-rs/config.yaml
notes_directory: /home/sackbuoy/Documents/notes
editor_command: nvim
config_file_path: /home/sackbuoy/.note-rs/config.yaml # wtf thats THIS file
extension: .md
```
