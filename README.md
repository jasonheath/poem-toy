# poem-toy
A quick and dirty toy app using the Poem web framework.

This is really just a grab of the static files example app from the poem
repository as it was on Oct 25, 2022 and modified to work as an standalone
app outside of the examples directory.  I also added a txt file with the 
text of Lewis Carroll's Jabberwocky because a web framework named "poem"
has to be able to return a poem. It insists upon it.

If you don't have rust...
- https://www.rust-lang.org/tools/install

Or to just cut to it if you're on a *nix and trusting...
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Once you have rust...
- clone https://github.com/jasonheath/poem-toy via your preferred method
- `cd poem-toy`
- `cargo run`
- http://127.0.0.1:3000
