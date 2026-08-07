#include <iostream>
#include <string>
#include <sstream>

// Version 3: Convert number to words in Spanish using native C++ library
// Usage: g++ version3.cpp -o version3 && ./version3 <number>
// Example: ./version3 10

std::string numberToSpanishWords(long long n) {
    if (n == 0) return "cero";
    
    std::string units[] = {"", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"};
    std::string teens[] = {"diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve"};
    std::string tens[] = {"", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"};
    std::string hundreds[] = {"", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"};
    
    if (n < 10) return units[n];
    if (n < 20) return teens[n - 10];
    if (n < 100) {
        int t = n / 10;
        int u = n % 10;
        if (u == 0) return tens[t];
        return tens[t] + " y " + units[u];
    }
    if (n < 1000) {
        int h = n / 100;
        long long rest = n % 100;
        if (h == 1 && rest == 0) return "cien";
        if (rest == 0) return hundreds[h];
        return hundreds[h] + " " + numberToSpanishWords(rest);
    }
    if (n < 1000000) {
        long long thousands = n / 1000;
        long long rest = n % 1000;
        if (thousands == 1) {
            if (rest == 0) return "mil";
            return "mil " + numberToSpanishWords(rest);
        }
        if (rest == 0) return numberToSpanishWords(thousands) + " mil";
        return numberToSpanishWords(thousands) + " mil " + numberToSpanishWords(rest);
    }
    
    std::stringstream ss;
    ss << n;
    return ss.str();
}

int main(int argc, char* argv[]) {
    std::string number = argc > 1 ? argv[1] : "10";
    
    try {
        long long num = std::stoll(number);
        std::string result = numberToSpanishWords(num);
        std::cout << result << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }
    
    return 0;
}
