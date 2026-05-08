# delete-empty-folders

[![Linux tests](https://github.com/kevgo/delete-empty-folders/actions/workflows/ci_linux.yml/badge.svg)](https://github.com/kevgo/delete-empty-folders/actions/workflows/ci_linux.yml)
[![Windows tests](https://github.com/kevgo/delete-empty-folders/actions/workflows/ci_windows.yml/badge.svg)](https://github.com/kevgo/delete-empty-folders/actions/workflows/ci_windows.yml)

Delete empty directories from the current directory tree
so your working tree matches what actually exists in Git.

A directory is considered empty if it contains no files, including nested files.
Directories that only contain other empty directories are also removed.

Does not scan or remove files
and folders [ignored by Git](https://git-scm.com/docs/gitignore).

## Why this exists

Git tracks files, not directories.

That means empty folders on your machine can stick around indefinitely,
even after switching branches, rebasing, or deleting files.
Since those folders are not part of the repository state:

* `git status` won't show them
* `git add` won't include them
* other developers, CI environments, and production servers won't have them

Over time, this can leave your local filesystem in a different state than every
other environment.

Most of the time that's harmless.
Sometimes it isn't.

Examples:

* code that scans directories and assumes their existence means something
* tools that require a particular folder to exist

For usage examples see the human-readable [end-to-end tests](features/).
