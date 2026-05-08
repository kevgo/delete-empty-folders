Feature: Deletes nested empty folders

  Scenario: a non-empty folder
    Given a file "folder/file.txt"
    When running delete-empty-folders
    Then it prints nothing
      """
      And the workspace is empty
      """
