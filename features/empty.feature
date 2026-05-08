Feature: Deletes empty folders

  Scenario: multiple empty subfolders
    Given a folder with:
      | folder | folderA/folder1 |
      | folder | folderA/folder2 |
      | folder | folderA/folder3 |
    When running delete-empty-folders
    Then it prints:
      """
      removing empty directory: folderA/folder1
      removing empty directory: folderA/folder2
      removing empty directory: folderA/folder3
      removing empty directory: folderA
      """
    And the workspace is empty

  Scenario: nested empty folders
    Given a folder with:
      | folder | folder1/folder2/folder3/folder4 |
    When running delete-empty-folders
    Then it prints:
      """
      removing empty directory: folder1/folder2/folder3/folder4
      removing empty directory: folder1/folder2/folder3
      removing empty directory: folder1/folder2
      removing empty directory: folder1
      """
    And the workspace is empty
