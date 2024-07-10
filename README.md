# just for fun :-)

## thirtyfour webdriver console for use XPath,Css and browser commands

## project init

```bash

cd # get to normal user home folder
mkdir thirtyfour_webdriver_console && cd $_ # create new project folder
cargo init . # initialisation project
cargo install cargo-edit # cargo edit already installed 
# for project
cargo add log # add log
cargo add env_logger
cargo add tokio --features full
# this should you see inside Cargo.toml
# tokio = { version = "1.38.0", features = ["full"] }
cargo add thirtyfour
# build project
cargo update --workspace --recursive --color auto --verbose
cargo build
```
## first interaktive user