Feature: Deletes empty folders

  Scenario: multiple empty subfolders
    Given a folder "folderA/folder1"
    And a folder "folderA/folder2"
    And a folder "folderA/folder3"
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
    Given a folder "folder1/folder2/folder3"
    When running delete-empty-folders
    Then it prints:
      """
      removing empty directory: folder1/folder2/folder3
      removing empty directory: folder1/folder2
      removing empty directory: folder1
      """
    And the workspace is empty
