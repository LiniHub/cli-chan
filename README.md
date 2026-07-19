# 🌺 cli-chan

Anime girl but with a command line interface.  
Supports Gemini API *for now*.  

Current Progress:  
- [x] Basic api usage.
- [x] TOML config file.
    - [x] Multiple personality support.
    - [ ] User config for each anime girl.
- [ ] Conversation history.
- [ ] Extended functionality.
- [ ] Extended api usage.

## How to install

### Source:

```bash
git clone https://github.com/LiniHub/cli-chan
cd cli-chan
cargo build --release
./target/release/cli-chan help
```

### Nix

```nix
inputs.cli-chan.url = "github:LiniHub/cli-chan";
```

```nix
environment.systemPackages = let
  system = pkgs.stdenv.hostPlatform.system;
in [
  inputs.cli-chan.packages.${system}.default
];
```

## Usage

```txt
A cli AI anime girl app for those who live in terminal.

Usage: cli-chan [OPTIONS] <COMMAND>

Commands:
  send  Send message to cli-chan
  help  Print this message or the help of the given subcommand(s)

Options:
  -a, --api-key <API_KEY>  API key
  -c, --config <CONFIG>    Config file
  -h, --help               Print help
  -V, --version            Print version
```

```txt
Send message to cli-chan

Usage: cli-chan send <ANIME_GIRL> <TEXT>

Arguments:
  <ANIME_GIRL>  Chose which anime girl you'll send the message to (noGirl to ignore config)
  <TEXT>        The string which you will send to the anime girl

Options:
  -h, --help  Print help
```

```bash
# use --api-key to set the api key inline or add it as GEMINI_API_KEY environment variable
# using `noGirl`  as anime_girl makes it ignore the config file
cli-chan send noGirl "what does くりかえす mean, do not use any markdown" 
cli-chan send yuno "How are you fairing my love?" 
```
