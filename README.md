This is a starting point for Rust solutions to the
["Build Your Own Git" Challenge](https://codecrafters.io/challenges/git).

In this challenge, you'll build a small Git implementation that's capable of
initializing a repository, creating commits and cloning a public repository.
Along the way we'll learn about the `.git` directory, Git objects (blobs,
commits, trees etc.), Git's transfer protocols and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to signup for early access.

# Usage

1. Ensure you have `cargo (1.54)` installed locally
1. Run `./your_git.sh` to run your Git implementation, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# Testing locally

The `your_git.sh` script is expected to operate on the `.git` folder inside the
current working directory. If you're running this inside the root of this
repository, you might end up accidentally damaging your repository's `.git`
folder.

We suggest executing `your_git.sh` in a different folder when testing locally.
For example:

```sh
mkdir -p /tmp/testing && cd /tmp/testing
/path/to/your/repo/your_git.sh init
```

To make this easier to type out, you could add a
[shell alias](https://shapeshed.com/unix-alias/):

```sh
alias mygit=/path/to/your/repo/your_git.sh

mkdir -p /tmp/testing && cd /tmp/testing
mygit init
```

# Passing the first stage

CodeCrafters runs tests when you do a `git push`. Make an empty commit and push
your solution to see the first stage fail.

```sh
git commit --allow-empty -m "Running tests"
git push origin master
```

You should see a failure message that says the `.git` directory wasn't
initialized.

Go to `src/main.rs` and uncomment the implementation for the `init` command.
Commit and push your changes to pass the first stage:

```sh
git add .
git commit -m "pass the first stage"
git push origin master
```

Time to move on to the next stage!
