const { Given, When, Then, AfterAll} = require('@cucumber/cucumber');
const axios = require('axios'); 
const assert = require('assert');

Given('username', function () {
  this.username = "test" + Date.now();  
});

Given('password', function () {
  this.password = "admin";
});

Given('password is too short', function () {
  this.password = "";
});

When('I try to create my account', function () {
  return axios.post('http://10.8.0.1:8080/v1/register',{
    username: this.username,
    password: this.password
  }).then((r) => {
    this.response = r;
  }).catch((e) => {
    this.response = e.response;
  })
});

Then('I should receive a status {int} response', function (int) {
  assert(this.response.status === int, `Status ${this.response.status} was different than ${int}`);
});