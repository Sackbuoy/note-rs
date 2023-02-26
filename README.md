# note-rs

Just a script for note taking that I wrote as a simple rust project bc I wanted to, and I use vim for everything else so it just made sense.

also, since I know you'll forget, to set up zsh completions:
1. Enable completions on your machine by adding this to your ~/.zshrc
```
autoload -U compinit
compinit
```
2. Add a `completions` dir somewhere in your PATH, then in that directory put a file called `_note`(or whatever the bin is prefixed with a \_) and put this in it:
```
#compdef _note note

function _note() {
  _arguments '1: :_path_files -W /path/to/notes/dir'
}
```
3. reload your shell and it should work

itd be cool if i could note hardcode that path lol but the config directory won't always be the same so i cant always pull from there. maybe and env var or something. great problem for future me
