@user
Feature: Add user account
  As a user I want to create my account

  Scenario: Create user account
    Given username
    And password
    When I try to create my account
    Then I should receive a successful response
