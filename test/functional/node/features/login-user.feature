@login
Feature: login user account
  As a user I want to login to my account

  Scenario: Create user account
    Given username
    And password
    When I try to create my account
    Then I try to login to my account
    Then I should receive a status 200 response