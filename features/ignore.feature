Feature: Default ignored directories

  Scenario: folder ".git" is ignored
    Given a folder with contents:
      | FOLDER | .git |
    And a .gitignore file with contents:
      """
      
      """
    When running delete-empty-folders
    Then it prints nothing
    And the workspace is unchanged

  Scenario: folder "node_modules" is ignored
    Given a folder with contents:
      | FOLDER | node_modules |
    When running delete-empty-folders
    Then it prints nothing
    And the workspace is unchanged
