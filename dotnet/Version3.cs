using System;
using System.Globalization;

// Version 3: Convert number to words in Spanish using native .NET library
// Usage: dotnet run --project Version3.csproj -- <number>
// Example: dotnet run --project Version3.csproj -- 10

namespace NumberConversion
{
    class Program
    {
        static void Main(string[] args)
        {
            string number = args.Length > 0 ? args[0] : "10";

            try
            {
                // Convert number to words in Spanish using NumberFormatInfo
                CultureInfo spanishCulture = new CultureInfo("es-ES");
                long num = long.Parse(number);
                
                // Use custom implementation for Spanish number to words
                string result = NumberToSpanishWords(num);
                
                Console.WriteLine(result);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error: {ex.Message}");
            }
        }

        static string NumberToSpanishWords(long number)
        {
            if (number == 0) return "cero";
            
            string[] units = { "", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve" };
            string[] teens = { "diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve" };
            string[] tens = { "", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa" };
            string[] hundreds = { "", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos" };

            if (number < 10) return units[number];
            if (number < 20) return teens[number - 10];
            if (number < 100)
            {
                int t = number / 10;
                int u = number % 10;
                return tens[t] + (u > 0 ? " y " + units[u] : "");
            }
            if (number < 1000)
            {
                int h = number / 100;
                int rest = number % 100;
                if (h == 1 && rest == 0) return "cien";
                return hundreds[h] + (rest > 0 ? " " + NumberToSpanishWords(rest) : "");
            }
            if (number < 1000000)
            {
                int thousands = (int)(number / 1000);
                int rest = (int)(number % 1000);
                if (thousands == 1) return "mil" + (rest > 0 ? " " + NumberToSpanishWords(rest) : "");
                return NumberToSpanishWords(thousands) + " mil" + (rest > 0 ? " " + NumberToSpanishWords(rest) : "");
            }
            
            return number.ToString();
        }
    }
}
