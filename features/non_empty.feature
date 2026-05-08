Feature: Does not delete non-empty folders

  Scenario: folder contains a file
    Given a file "folder/file.txt"
    When running delete-empty-folders
    Then it prints nothing
    And the workspace is unchanged
