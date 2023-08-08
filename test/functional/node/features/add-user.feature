@user
Feature: Add user account
  As a user I want to create my account

  Scenario: Create user account
    Given username
    And password
    When I try to create my account
    Then I should receive a status 201 response

  Scenario: User password length must be larger than 3
    Given username
    And password is too short
    When I try to create my account
    Then I should receive a status 400 response
