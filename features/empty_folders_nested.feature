Feature: Deletes nested empty folders

  Scenario: two nested empty folders
    Given a folder "folder1/folder2/folder3"
    When running delete-empty-folders
    Then it prints:
      """
      removing empty directory: folder1/folder2/folder3
      removing empty directory: folder1/folder2
      removing empty directory: folder1
      """
    And the workspace is empty
