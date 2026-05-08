Feature: Default ignored directories

  Scenario: folder ".git" is ignored
    Given a folder with:
      | folder | .git |
    When running delete-empty-folders
    Then it prints nothing
    And the workspace is unchanged

  Scenario: folder "node_modules" is ignored
    Given a folder with:
      | folder | node_modules |
    When running delete-empty-folders
    Then it prints nothing
    And the workspace is unchanged
