#!/usr/bin/perl
# Version 3: Convert number to words in Spanish using native Perl library
# Usage: perl version3.pl <number>
# Example: perl version3.pl 10

use strict;
use warnings;
use Lingua::ES::Numeros;

# Get number from command line argument
my $number = $ARGV[0] || '10';

# Convert number to words in Spanish
my $converter = Lingua::ES::Numeros->new;
my $result = $converter->cardinal($number);

print "$result\n";
