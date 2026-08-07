#!/usr/bin/env ruby
# Version 2: Consume SOAP service + translate to Spanish
# Usage: ruby version2.rb <number>
# Example: ruby version2.rb 10

require 'savon'
require 'google_translate'

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

# Extract the result
result = response.body[:number_to_words_response][:number_to_words_result]

# Translate from English to Spanish
translator = GoogleTranslate.new
translated = translator.translate(result, from: 'en', to: 'es')

puts translated
