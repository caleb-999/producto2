#!/usr/bin/env ruby
# Version 1: Consume SOAP service directly
# Usage: ruby version1.rb <number>
# Example: ruby version1.rb 10

require 'savon'

# Get number from command line argument
number = ARGV[0] || '10'

# Configure SOAP client
client = Savon.client(
  wsdl: 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL',
  log: false
)

# Call the NumberToWords operation
response = client.call(
  :number_to_words,
  message: { 'ubiNum' => number }
)

# Extract and display the result
result = response.body[:number_to_words_response][:number_to_words_result]
puts result
