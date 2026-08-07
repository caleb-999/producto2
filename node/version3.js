#!/usr/bin/env node
// Version 3: Convert number to words in Spanish using native Node.js library
// Usage: node version3.js <number>
// Example: node version3.js 10

const n2words = require('n2words');

// Get number from command line argument
const number = parseInt(process.argv[2]) || 10;

// Convert number to words in Spanish
const result = n2words(number, { lang: 'es' });

console.log(result);
