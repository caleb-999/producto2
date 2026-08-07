import javax.xml.namespace.QName;
import jakarta.xml.ws.Service;
import java.net.URL;
import com.google.cloud.translate.Translate;
import com.google.cloud.translate.TranslateOptions;
import com.google.cloud.translate.Translation;

// Version 2: Consume SOAP service + translate to Spanish
// Usage: javac Version2.java && java Version2 <number>
// Example: java Version2 10

public class Version2 {
    public static void main(String[] args) {
        String number = args.length > 0 ? args[0] : "10";
        
        try {
            URL url = new URL("https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL");
            QName qname = new QName("http://www.dataaccess.com/webservicesserver/", "NumberConversion");
            
            Service service = Service.create(url, qname);
            NumberConversion proxy = service.getPort(NumberConversion.class);
            
            String result = proxy.numberToWords(Long.parseLong(number));
            
            // Translate from English to Spanish
            Translate translate = TranslateOptions.getDefaultInstance().getService();
            Translation translation = translate.translate(result, Translate.TranslateOption.targetLanguage("es"));
            
            System.out.println(translation.getTranslatedText());
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
        }
    }
}

interface NumberConversion {
    String numberToWords(long ubiNum);
}
