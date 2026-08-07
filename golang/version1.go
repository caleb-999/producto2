package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/hooklift/gowsdl/soap"
)

// Version 1: Consume SOAP service directly
// Usage: go run version1.go <number>
// Example: go run version1.go 10

type NumberConversion struct {
	Client *soap.Client
}

type NumberToWords struct {
	UbiNum uint64 `xml:"ubiNum"`
}

type NumberToWordsResponse struct {
	NumberToWordsResult string `xml:"NumberToWordsResult"`
}

func main() {
	number := "10"
	if len(os.Args) > 1 {
		number = os.Args[1]
	}

	client := soap.NewClient("https://www.dataaccess.com/webservicesserver/NumberConversion.wso", nil)
	
	service := &NumberConversion{Client: client}
	
	req := &NumberToWords{}
	req.UbiNum, _ = strconv.ParseUint(number, 10, 64)
	
	resp := &NumberToWordsResponse{}
	err := client.Call("NumberToWords", req, resp)
	
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}
	
	fmt.Println(resp.NumberToWordsResult)
}
