import java.text.NumberFormat;
import java.util.Locale;

// Version 3: Convert number to words in Spanish using native Java library
// Usage: javac Version3.java && java Version3 <number>
// Example: java Version3 10

public class Version3 {
    public static void main(String[] args) {
        String number = args.length > 0 ? args[0] : "10";
        
        try {
            long num = Long.parseLong(number);
            
            // Use NumberFormat with Spanish locale for spellout
            Locale spanishLocale = new Locale("es", "ES");
            NumberFormat formatter = NumberFormat.getInstance(spanishLocale);
            
            // Note: Java's NumberFormat doesn't support spellout directly
            // We'll use a custom implementation for Spanish number to words
            String result = numberToSpanishWords(num);
            
            System.out.println(result);
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
        }
    }
    
    private static String numberToSpanishWords(long n) {
        if (n == 0) return "cero";
        
        String[] units = {"", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve"};
        String[] teens = {"diez", "once", "doce", "trece", "catorce", "quince", "dieciséis", "diecisiete", "dieciocho", "diecinueve"};
        String[] tens = {"", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa"};
        String[] hundreds = {"", "ciento", "doscientos", "trescientos", "cuatrocientos", "quinientos", "seiscientos", "setecientos", "ochocientos", "novecientos"};
        
        if (n < 10) return units[(int)n];
        if (n < 20) return teens[(int)(n - 10)];
        if (n < 100) {
            int t = (int)(n / 10);
            int u = (int)(n % 10);
            if (u == 0) return tens[t];
            return tens[t] + " y " + units[u];
        }
        if (n < 1000) {
            int h = (int)(n / 100);
            long rest = n % 100;
            if (h == 1 && rest == 0) return "cien";
            if (rest == 0) return hundreds[h];
            return hundreds[h] + " " + numberToSpanishWords(rest);
        }
        if (n < 1000000) {
            long thousands = n / 1000;
            long rest = n % 1000;
            if (thousands == 1) {
                if (rest == 0) return "mil";
                return "mil " + numberToSpanishWords(rest);
            }
            if (rest == 0) return numberToSpanishWords(thousands) + " mil";
            return numberToSpanishWords(thousands) + " mil " + numberToSpanishWords(rest);
        }
        
        return String.valueOf(n);
    }
}
