using System;
using System.ServiceModel;

// Version 1: Consume SOAP service directly
// Usage: dotnet run --project Version1.csproj -- <number>
// Example: dotnet run --project Version1.csproj -- 10

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
                Console.WriteLine(result);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error: {ex.Message}");
            }
        }
    }
}
