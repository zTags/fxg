# fxg

## Your first FXG blog

You will need

- Cargo
- A text editor ([VSCode](https://code.visualstudio.com/) is recommended, but you can use anything)
- 2 tbsp salt

Steps:

1. Open a terminal
2. In this terminal, run the command `cargo +nightly install fxg` (you need to download [rust nightly](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust) for this)
3. When this finishes, run `fxg new my_first_fxg_blog`
4. Open VSCode in the newly created folder.
5. Press `ctrl+P` and write `ext install tags.fxg-language-support`
6. All features are used in this one file, you can view this as a [cheatsheet](https://gist.github.com/zTags/ba3f4ef67a1593f1b71fa33edcebaa2e)
7. run `fxg build --start` to run a webserver with your blog
