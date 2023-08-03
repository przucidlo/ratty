const { Given, When, Then } = require('@cucumber/cucumber');
const axios = require('axios'); 
const assert = require('assert');

let username;
let password;
let response;

Given('username', function () {
  username = "test" + Date.now();  
});

Given('password', function () {
  password = "admin";
});

When('I try to create my account', function () {
  return axios({
    method: 'post',
    url: 'http://10.8.0.1:8080/v1/register', 
    data: {
      username: username,
      password: password
    },
    timeout: 3000,
  }).then((r) => {
    response = r;
  })
});

Then('I should receive a successful response', function () {
  assert(response.status === 201, "Status was different than 201");
});