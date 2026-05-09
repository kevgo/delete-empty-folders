Feature: Prints help

  Scenario: short flag
    When running "delete-empty-folders -h"
    Then it prints:
      """
      Deletes all empty directories in the current directory and its subdirectories.
      
      Usage: delete-empty-folders
      """

  Scenario: long flag
    When running "delete-empty-folders --help"
    Then it prints:
      """
      Deletes all empty directories in the current directory and its subdirectories.
      
      Usage: delete-empty-folders
      """
    Given a folder with contents
