# Barney's Rust Playground

This project is... a playground... for Rust... for Barney. Shocking, I know.

In case it doesn't go without saying: please don't assume anything you find in
here is good. Even _I_ don't think it's good, and I wrote it to the best of my
ability (at the time). There are a lot of inconsistencies, stylistic and
otherwise. There's also stuff that is non-idiomatic, and even flat-out wrong.
The goal is for me to play and learn, not to build a good software package.

## Running

Do the `rustup` dance, then `cargo run`. You know the drill. You'll get a nice
little interactive menu to run one of several utilities, all of which are of
_extremely_ little value. Except to me. For playing!

If you want to jump directly into a utility, you can supply its name on the
command line:

    cargo run guess

## The Advent of Code Utilities

I like [Advent of Code](https://adventofcode.com). You should give it a try if
you haven't before. As well as entertaining me all December, the problems also
provide nice ready-made sample apps of low - but non-trivial - complexity for
playing with stuff. Like Rust!

The `aoc_*` utilities require an input file to read from, by default the same as
the utility name with a `.txt` extension in the current working directory. Or
they will accept a filename on the command line, after the utility name, if you
don't have your input file named right. That is, these two are equivalent (but
note that an input file is required either way):

    cargo run aoc_2019_01
    cargo run aoc_2019_01 aoc_2019_01.txt

I opted for the above naming convention as the input filenames give a good
approximation to subcommand tab-completion without having to do any work. E.g.
after dropping a default-named input file, enter `cargo run a` and hit TAB. Now
you just have to delete the `.txt` and you're done. It gets better if you have
multiple files. If you don't like my naming conventions, don't use them; the
filename can be anything:

    mv aoc_2019_01.txt input.txt
    cargo run aoc_2019_01 input.txt

No pipes though. Sorry, man.

For Advent of Code problems that don't supply input file (e.g., 2019 day 4),
copy and paste your input into a file.

## Profiling

This is potentially more interesting, and potentially foolish and pointless:

    cargo build
    valgrind --tool=massif \
        --stacks=yes \
        --massif-out-file=massif.out \
        --detailed-freq=10000 \
        ./target/debug/rust_playground aoc_2019_03
    ms_print massif.out | less
    
As is:

    /usr/bin/time ./target/debug/rust_playground aoc_2019_03

I'm quite confident this is rather far from ideal. I'm not a systems programmer,
so I'm figuring it out as I go. They did, however, help me reduce runtime of the
`aoc_2019_03` utility by about 20% and heap usage by about 30%.
