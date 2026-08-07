#!/usr/bin/env node
// Version 2: Consume SOAP service + translate to Spanish
// Usage: node version2.js <number>
// Example: node version2.js 10

const soap = require('soap');
const { Translate } = require('@google-cloud/translate').v2;

// Get number from command line argument
const number = process.argv[2] || '10';

// Configure SOAP client
const url = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';

soap.createClient(url, async (err, client) => {
    if (err) {
        console.error('Error creating SOAP client:', err);
        return;
    }

    // Call the NumberToWords operation
    client.NumberToWords({ ubiNum: number }, async (err, result) => {
        if (err) {
            console.error('Error calling NumberToWords:', err);
            return;
        }

        // Translate from English to Spanish
        const translate = new Translate();
        const [translation] = await translate.translate(result.NumberToWordsResult, 'es');
        
        console.log(translation);
    });
});
