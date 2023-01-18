# rmsafe (remove_safely)

## Introduction

After (almost) accidentally removing a file with about week's worth of changes for my C lang assignment, I took it as an excuse to lear a bit
more of Rust and re-write the rm command.

## Basics

Instead of just rm(ing) files and folders, they instead are moved to the local trash or custom path location.

## Installation
`cargo install rmsafe`

## Usage

NOTE: I have only tested this on Linux Mint 20.3. 

```
// view trashcan path
rmsafe 

// removing a single file
rmsafe test.txt

// removing a single folder; it will recursively move the folder to trash
rmsafe test_dir

// removing files with wildcard matching; removing all files ending with .o
rmsafe -r "*.o"  

// change trashcan path
rmsafe -t "/home/jane/Desktop/.rmsafe"
```

Change your `.bashrc` to include the following
```
alias rm='printf "Avoid using rm!"'
alias rms="rmsafe"
```

This disallows the use of rm, you can still use `sudo rm` and not setting rmsafe to rm
ensures that you don't accidentally rm while on someone else's computer

NOTE: The shell might interpret wildcards before they are passed down as function arguments which might cause problems, best to disable wildcard expansion on the shell

## Contribution

Open up an issue on GitHub and I'll be in touch!
