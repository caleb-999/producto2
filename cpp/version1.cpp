#include <iostream>
#include <string>
#include <curl/curl.h>
#include <tinyxml2.h>

// Version 1: Consume SOAP service directly
// Usage: g++ version1.cpp -lcurl -ltinyxml2 -o version1 && ./version1 <number>
// Example: ./version1 10

size_t WriteCallback(void* contents, size_t size, size_t nmemb, void* userp) {
    ((std::string*)userp)->append((char*)contents, size * nmemb);
    return size * nmemb;
}

int main(int argc, char* argv[]) {
    std::string number = argc > 1 ? argv[1] : "10";
    
    CURL* curl = curl_easy_init();
    if(curl) {
        std::string soapRequest = 
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>"
            "<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">"
            "<soap:Body>"
            "<NumberToWords xmlns=\"http://www.dataaccess.com/webservicesserver/\">"
            "<ubiNum>" + number + "</ubiNum>"
            "</NumberToWords>"
            "</soap:Body>"
            "</soap:Envelope>";
        
        std::string response;
        
        curl_easy_setopt(curl, CURLOPT_URL, "https://www.dataaccess.com/webservicesserver/NumberConversion.wso");
        curl_easy_setopt(curl, CURLOPT_POST, 1L);
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, soapRequest.c_str());
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);
        
        struct curl_slist* headers = NULL;
        headers = curl_slist_append(headers, "Content-Type: text/xml; charset=utf-8");
        headers = curl_slist_append(headers, "SOAPAction: \"http://www.dataaccess.com/webservicesserver/NumberToWords\"");
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        
        CURLcode res = curl_easy_perform(curl);
        
        if(res != CURLE_OK) {
            std::cerr << "curl_easy_perform() failed: " << curl_easy_strerror(res) << std::endl;
        } else {
            tinyxml2::XMLDocument doc;
            doc.Parse(response.c_str());
            
            tinyxml2::XMLElement* root = doc.FirstChildElement();
            if(root) {
                tinyxml2::XMLElement* body = root->FirstChildElement("soap:Body");
                if(body) {
                    tinyxml2::XMLElement* response = body->FirstChildElement("m:NumberToWordsResponse");
                    if(response) {
                        tinyxml2::XMLElement* result = response->FirstChildElement("m:NumberToWordsResult");
                        if(result) {
                            std::cout << result->GetText() << std::endl;
                        }
                    }
                }
            }
        }
        
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
    }
    
    return 0;
}
