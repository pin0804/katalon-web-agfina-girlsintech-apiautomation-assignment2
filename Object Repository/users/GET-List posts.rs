<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-List posts</name>
   <tag></tag>
   <elementGuidId>1547aea7-eb00-4e0e-92a3-614f4f47943d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b48d27fd-023f-4eb2-a602-240688a4ed5e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/posts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, '[0].userId', '')
WS.verifyElementPropertyValue(response, '[0].id', '')
WS.verifyElementPropertyValue(response, '[0].title', '')
WS.verifyElementPropertyValue(response, '[0].body', '')

WS.verifyElementPropertyValue(response, '[1].userId', '')
WS.verifyElementPropertyValue(response, '[1].id', '')
WS.verifyElementPropertyValue(response, '[1].title', '')
WS.verifyElementPropertyValue(response, '[1].body', '')

WS.verifyElementPropertyValue(response, '[2].userId', '')
WS.verifyElementPropertyValue(response, '[2].id', '')
WS.verifyElementPropertyValue(response, '[2].title', '')
WS.verifyElementPropertyValue(response, '[2].body', '')

WS.verifyElementPropertyValue(response, '[3].userId', '')
WS.verifyElementPropertyValue(response, '[3].id', '')
WS.verifyElementPropertyValue(response, '[3].title', '')
WS.verifyElementPropertyValue(response, '[3].body', '')

WS.verifyElementPropertyValue(response, '[4].userId', '')
WS.verifyElementPropertyValue(response, '[4].id', '')
WS.verifyElementPropertyValue(response, '[4].title', '')
WS.verifyElementPropertyValue(response, '[4].body', '')

WS.verifyElementPropertyValue(response, '[5].userId', '')
WS.verifyElementPropertyValue(response, '[5].id', '')
WS.verifyElementPropertyValue(response, '[5].title', '')
WS.verifyElementPropertyValue(response, '[5].body', '')

WS.verifyElementPropertyValue(response, '[6].userId', '')
WS.verifyElementPropertyValue(response, '[6].id', '')
WS.verifyElementPropertyValue(response, '[6].title', '')
WS.verifyElementPropertyValue(response, '[6].body', '')

WS.verifyElementPropertyValue(response, '[7].userId', '')
WS.verifyElementPropertyValue(response, '[7].id', '')
WS.verifyElementPropertyValue(response, '[7].title', '')
WS.verifyElementPropertyValue(response, '[7].body', '')

WS.verifyElementPropertyValue(response, '[8].userId', '')
WS.verifyElementPropertyValue(response, '[8].id', '')
WS.verifyElementPropertyValue(response, '[8].title', '')
WS.verifyElementPropertyValue(response, '[8].body', '')

WS.verifyElementPropertyValue(response, '[9].userId', '')
WS.verifyElementPropertyValue(response, '[9].id', '')
WS.verifyElementPropertyValue(response, '[9].title', '')
WS.verifyElementPropertyValue(response, '[9].body', '')

WS.verifyElementPropertyValue(response, '[10].userId', '')
WS.verifyElementPropertyValue(response, '[10].id', '')
WS.verifyElementPropertyValue(response, '[10].title', '')
WS.verifyElementPropertyValue(response, '[10].body', '')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
