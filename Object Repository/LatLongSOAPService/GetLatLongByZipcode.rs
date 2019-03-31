<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetLatLongByZipcode</name>
   <tag></tag>
   <elementGuidId>8884ba5f-4ae6-4626-be4b-b647542321fa</elementGuidId>
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
        &lt;LatLonListZipCode xmlns=&quot;https://graphical.weather.gov/xml/DWMLgen/wsdl/ndfdXML.wsdl&quot;>
            &lt;zipCodeList>${zipcode}&lt;/zipCodeList>
        &lt;/LatLonListZipCode>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>LatLonListZipCode</soapServiceFunction>
   <variables>
      <defaultValue>90210</defaultValue>
      <description></description>
      <id>f1c27e38-460c-43a7-9436-95900ca2bf02</id>
      <masked>false</masked>
      <name>zipcode</name>
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

WS.verifyElementText(response, 'LatLonListZipCodeResponse.listLatLonOut', '34.0995,-118.414')</verificationScript>
   <wsdlAddress>https://graphical.weather.gov/xml/SOAP_server/ndfdXMLserver.php?wsdl</wsdlAddress>
</WebServiceRequestEntity>
