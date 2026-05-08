Feature: Default ignored directories

  Scenario: .git is ignored
    Given a folder ".git"
    When running delete-empty-folders
    Then it prints nothing
    And the workspace is unchanged

  Scenario: node_modules is ignored
    Given a folder "node_modules"
    When running delete-empty-folders
    Then it prints nothing
    And the workspace is unchanged
