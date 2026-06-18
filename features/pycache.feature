Feature: Deletes folders containing only __pycache__

  @this
  Scenario: folder contains only __pycache__
    Given a folder with contents:
      | FILE | folder/__pycache__/file.pyc |
    When running "delete-empty-folders"
    Then it prints:
      """
      removing empty directory: folder
      """
    And the workspace is empty
