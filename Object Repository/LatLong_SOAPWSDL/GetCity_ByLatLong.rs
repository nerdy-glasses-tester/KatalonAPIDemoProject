<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetCity_ByLatLong</name>
   <tag></tag>
   <elementGuidId>69834466-a0f0-4494-b6b5-f8ca78322fc6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;LatLonListCityNames xmlns=&quot;https://graphical.weather.gov/xml/DWMLgen/wsdl/ndfdXML.wsdl&quot;>
            &lt;displayLevel>${lat}&lt;/displayLevel>
        &lt;/LatLonListCityNames>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>LatLonListCityNames</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.lat</defaultValue>
      <description></description>
      <id>9de10ace-4fdf-4f61-9604-0dae6aa6ee03</id>
      <masked>false</masked>
      <name>lat</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)
assertThat(response.getResponseText()).contains(&quot;Palm Springs,CA&quot;)
//wsdl has issues so can't use below
//WS.verifyElementText(response, 'LatLonListCityNamesResponse.listLatLonOut', 'Palm Springs,CA')
</verificationScript>
   <wsdlAddress>https://graphical.weather.gov/xml/SOAP_server/ndfdXMLserver.php?wsdl</wsdlAddress>
</WebServiceRequestEntity>
