import javax.xml.namespace.QName;
import jakarta.xml.ws.Service;
import java.net.URL;

// Version 1: Consume SOAP service directly
// Usage: javac Version1.java && java Version1 <number>
// Example: java Version1 10

public class Version1 {
    public static void main(String[] args) {
        String number = args.length > 0 ? args[0] : "10";
        
        try {
            URL url = new URL("https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL");
            QName qname = new QName("http://www.dataaccess.com/webservicesserver/", "NumberConversion");
            
            Service service = Service.create(url, qname);
            NumberConversion proxy = service.getPort(NumberConversion.class);
            
            String result = proxy.numberToWords(Long.parseLong(number));
            System.out.println(result);
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
        }
    }
}

interface NumberConversion {
    String numberToWords(long ubiNum);
}
