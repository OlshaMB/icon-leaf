# icon-leaf
### Only mac-yabai is currently supported
icon-leaf is an app that takes app names for some desktop and takes icons from config file, and matches them for the symbol(nerd tree icon) an outputs them.
## Installation
WIP
You need to build yourself:
1. Install rust
2. run `cargo build --release`
3. move binary `target/release/icon-leaf` `to /usr/local/bin/` or your bin dir
## Usage
1. Install appleaf on to your system see [Installation](#Installation)
2. configure apps in the appsdb config
Appsdb is located in 
MacOS: `~/Library/Application Support/icon-leaf/appsdb`
Linux: `~/.config/icon-leaf/appsdb`
appsdb is a csv file:
```json
app_name,icon
"App Name",i
"App Name2",n
```
where `i`/`n` is a utf-8 char(nerd font icon)
[example config](https://github.com/OlshaMB/icon-leaf/blob/master/example/appsdb/default-olshamb)
You can pull request yours.
3. test it by running
`icon-leaf <desktop id to list apps>` 
4. Configure it for your bar
#### example configs
[`sketchybar - space.sh`](https://gist.github.com/ef223f788d94198a400139a8acb5b39a) - yabai  space script config
You can pull request yours.
