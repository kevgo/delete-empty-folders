Feature: Does not delete non-empty folders

  Scenario: folder contains a file
    Given a folder with contents:
      | FILE | folder/file.txt |
    When running delete-empty-folders
    Then it prints nothing
    And the workspace is unchanged

  Scenario: folder contains a file and an empty subfolder
    Given a folder with contents:
      | FILE   | folder/file.txt        |
      | FOLDER | folder/empty_subfolder |
    When running delete-empty-folders
    Then it prints:
      """
      removing empty directory: folder/empty_subfolder
      """
    And the workspace contains:
      | FOLDER | folder          |
      | FILE   | folder/file.txt |
