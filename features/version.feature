Feature: Prints version

  Scenario: short flag
    When running "delete-empty-folders -V"
    Then it prints:
      """
      delete-empty-folders 0.0.2
      """

  Scenario: long flag
    When running "delete-empty-folders --version"
    Then it prints:
      """
      delete-empty-folders 0.0.2
      """
