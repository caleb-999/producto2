package main

import (
	"context"
	"fmt"
	"os"
	"strconv"

	"github.com/hooklift/gowsdl/soap"
	"cloud.google.com/go/translate"
	"google.golang.org/api/option"
)

// Version 2: Consume SOAP service + translate to Spanish
// Usage: go run version2.go <number>
// Example: go run version2.go 10

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
	
	// Translate from English to Spanish
	ctx := context.Background()
	translateClient, err := translate.NewClient(ctx, option.WithAPIKey("YOUR_API_KEY"))
	if err != nil {
		fmt.Printf("Error creating translation client: %v\n", err)
		return
	}
	defer translateClient.Close()
	
	translations, err := translateClient.Translate(ctx, []string{resp.NumberToWordsResult}, "es", nil)
	if err != nil {
		fmt.Printf("Error translating: %v\n", err)
		return
	}
	
	fmt.Println(translations[0].Text)
}
