# delete-empty-folders

This tool deletes folders in the current directory or subdirectories that contain no files.
Folders containing only other empty folders are considered empty.

## Q&A

### Why?

Empty folders can lead to a different folder structure on your dev machine vs other dev machines, CI servers, or production.
This can lead to errors.
Using this tool avoids those errors.

This happens because Git doesntracks only files, not folders.
When Git switches to a different branch, it creates and removes folders as necessary to create the files on that branch.

If your workstation contains empty folders,
those folders will not be stored in Git.
Running `git status` will not show them and `git add` ignore them.
This means other developers and cloud servers will see a different folder structure than you have on your machine.
This can cause different behavior on your machine vs other machines.
