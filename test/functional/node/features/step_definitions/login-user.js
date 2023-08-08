const { Given, When, Then } = require('@cucumber/cucumber');
const axios = require('axios'); 
const assert = require('assert');

Then('I try to login to my account', function () {  
  return axios.post('http://10.8.0.1:8080/v1/authorize',{
    username: this.username,
    password: this.password
  }).then((r) => {
    this.response = r;
  }).catch((e) => {
    this.response = e.response;
  })
});