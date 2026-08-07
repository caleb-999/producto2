#!/usr/bin/perl
# Version 2: Consume SOAP service + translate to Spanish
# Usage: perl version2.pl <number>
# Example: perl version2.pl 10

use strict;
use warnings;
use SOAP::Lite;
use Lingua::Translate;

# Get number from command line argument
my $number = $ARGV[0] || '10';

# Configure SOAP client
my $wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';
my $client = SOAP::Lite->service($wsdl);

# Call the NumberToWords operation
my $result = $client->NumberToWords($number);

# Translate from English to Spanish
my $translator = Lingua::Translate->new(src => 'en', dest => 'es');
my $translated = $translator->translate($result);

print "$translated\n";
