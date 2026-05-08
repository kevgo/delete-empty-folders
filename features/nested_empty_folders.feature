Feature: Deletes nested empty folders

  Scenario: two nested empty folders
    Given a folder "folder/subfolder"
    When running delete-empty-folders
    Then it prints:
      """
      removing empty directory: folder/subfolder
      removing empty directory: folder
      """
    And the workspace is empty
