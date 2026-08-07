#!/usr/bin/env node
// Version 1: Consume SOAP service directly
// Usage: node version1.js <number>
// Example: node version1.js 10

const soap = require('soap');

// Get number from command line argument
const number = process.argv[2] || '10';

// Configure SOAP client
const url = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';

soap.createClient(url, (err, client) => {
    if (err) {
        console.error('Error creating SOAP client:', err);
        return;
    }

    // Call the NumberToWords operation
    client.NumberToWords({ ubiNum: number }, (err, result) => {
        if (err) {
            console.error('Error calling NumberToWords:', err);
            return;
        }

        console.log(result.NumberToWordsResult);
    });
});
