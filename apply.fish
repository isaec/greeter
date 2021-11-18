#!/usr/bin/env fish

cargo build --release

sudo install target/release/greeter /usr/local/bin/
echo "function fish_greeting
  greeter
end" >~/.config/fish/functions/fish_greeting.fish

source ~/.config/fish/functions/fish_greeting.fish
