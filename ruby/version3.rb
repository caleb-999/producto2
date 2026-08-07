#!/usr/bin/env ruby
# Version 3: Convert number to words in Spanish using native Ruby library
# Usage: ruby version3.rb <number>
# Example: ruby version3.rb 10

require 'i18n'
require 'i18n/backend/fallbacks'

# Configure I18n
I18n::Backend::Simple.include(I18n::Backend::Fallbacks)
I18n.load_path += Dir[File.expand_path('config/locales') + '/*.yml']
I18n.backend.load_translations
I18n.default_locale = :es

# Get number from command line argument
number = ARGV[0] || '10'

# Convert number to words in Spanish using humanize gem
require 'humanize'
result = number.to_i.humanize

puts result
