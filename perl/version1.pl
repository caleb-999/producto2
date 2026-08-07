#!/usr/bin/perl
# Version 1: Consume SOAP service directly
# Usage: perl version1.pl <number>
# Example: perl version1.pl 10

use strict;
use warnings;
use SOAP::Lite;

# Get number from command line argument
my $number = $ARGV[0] || '10';

# Configure SOAP client
my $wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';
my $client = SOAP::Lite->service($wsdl);

# Call the NumberToWords operation
my $result = $client->NumberToWords($number);

# Display the result
print "$result\n";
