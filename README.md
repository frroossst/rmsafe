# rmsafe (remove_safely)

## Introduction

After (almost) accidentally removing a file with about week's worth of changes for my C lang assignment, I took it as an excuse to lear a bit
more of Rust and re-write the rm command.

## Basics

Instead of just rm(ing) files and folders, they instead are moved to the local trash or custom path location.

## Installation
`cargo install rmsafe`

## Usage

NOTE: I have only tested this on Linux. 

```
// removing a single file
rmsafe -f test.txt

// removing a single folder; it will recursively move the folder to trash
rmsafe -f test_dir

// removing files with wildcard matching; removing all files ending with .o
rmsafe -r "*.o"  
```