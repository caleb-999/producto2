using System;
using System.ServiceModel;
using Google.Cloud.Translation.V2;

// Version 2: Consume SOAP service + translate to Spanish
// Usage: dotnet run --project Version2.csproj -- <number>
// Example: dotnet run --project Version2.csproj -- 10

namespace NumberConversion
{
    [ServiceContract]
    public interface INumberConversion
    {
        [OperationContract]
        string NumberToWords(ulong ubiNum);
    }

    class Program
    {
        static void Main(string[] args)
        {
            string number = args.Length > 0 ? args[0] : "10";

            BasicHttpBinding binding = new BasicHttpBinding();
            EndpointAddress address = new EndpointAddress("https://www.dataaccess.com/webservicesserver/NumberConversion.wso");
            
            ChannelFactory<INumberConversion> factory = new ChannelFactory<INumberConversion>(binding, address);
            INumberConversion client = factory.CreateChannel();

            try
            {
                string result = client.NumberToWords(ulong.Parse(number));
                
                // Translate from English to Spanish
                TranslationClient translationClient = TranslationClient.Create();
                var response = translationClient.TranslateText(result, "es", "en");
                
                Console.WriteLine(response.TranslatedText);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error: {ex.Message}");
            }
        }
    }
}
