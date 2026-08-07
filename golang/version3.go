package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
)

// Version 3: Convert number to words in Spanish using native Go library
// Usage: go run version3.go <number>
// Example: go run version3.go 10

func main() {
	number := "10"
	if len(os.Args) > 1 {
		number = os.Args[1]
	}

	num, err := strconv.ParseInt(number, 10, 64)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	result := numberToSpanishWords(num)
	fmt.Println(result)
}

func numberToSpanishWords(n int64) string {
	if n == 0 {
		return "cero"
	}

	units := []string{"", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"}
	teens := []string{"diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve"}
	tens := []string{"", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"}
	hundreds := []string{"", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"}

	if n < 10 {
		return units[n]
	}
	if n < 20 {
		return teens[n-10]
	}
	if n < 100 {
		t := n / 10
		u := n % 10
		if u == 0 {
			return tens[t]
		}
		return tens[t] + " y " + units[u]
	}
	if n < 1000 {
		h := n / 100
		rest := n % 100
		if h == 1 && rest == 0 {
			return "cien"
		}
		if rest == 0 {
			return hundreds[h]
		}
		return hundreds[h] + " " + numberToSpanishWords(rest)
	}
	if n < 1000000 {
		thousands := n / 1000
		rest := n % 1000
		if thousands == 1 {
			if rest == 0 {
				return "mil"
			}
			return "mil " + numberToSpanishWords(rest)
		}
		if rest == 0 {
			return numberToSpanishWords(thousands) + " mil"
		}
		return numberToSpanishWords(thousands) + " mil " + numberToSpanishWords(rest)
	}

	return strconv.FormatInt(n, 10)
}
