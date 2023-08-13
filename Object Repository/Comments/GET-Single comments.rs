<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-Single comments</name>
   <tag></tag>
   <elementGuidId>63f43d59-aac3-4e3c-aaf0-fab8ee8993c9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/posts/1/comments</restUrl>
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


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))

WS.verifyElementPropertyValue(response, '[0].postId', 1)
WS.verifyElementPropertyValue(response, '[0].id', 1)
WS.verifyElementPropertyValue(response, '[0].name', 'id labore ex et quam laborum')
WS.verifyElementPropertyValue(response, '[0].email', 'Eliseo@gardner.biz')
WS.verifyElementPropertyValue(response, '[0].body', 'laudantium enim quasi est quidem magnam voluptate ipsam eos\ntempora quo necessitatibus\ndolor quam autem quasi\nreiciendis et nam sapiente accusantium')

WS.verifyElementPropertyValue(response, '[1].postId', 1)
WS.verifyElementPropertyValue(response, '[1].id', 2)
WS.verifyElementPropertyValue(response, '[1].name', 'quo vero reiciendis velit similique earum')
WS.verifyElementPropertyValue(response, '[1].email', 'Jayne_Kuhic@sydney.com')
WS.verifyElementPropertyValue(response, '[1].body', 'est natus enim nihil est dolore omnis voluptatem numquam\net omnis occaecati quod ullam at\nvoluptatem error expedita pariatur\nnihil sint nostrum voluptatem reiciendis et')

WS.verifyElementPropertyValue(response, '[2].postId', 1)
WS.verifyElementPropertyValue(response, '[2].id', 3)
WS.verifyElementPropertyValue(response, '[2].name', 'odio adipisci rerum aut animi')
WS.verifyElementPropertyValue(response, '[2].email', 'Nikita@garfield.biz')
WS.verifyElementPropertyValue(response, '[2].body', 'quia molestiae reprehenderit quasi aspernatur\naut expedita occaecati aliquam eveniet laudantium\nomnis quibusdam delectus saepe quia accusamus maiores nam est\ncum et ducimus et vero voluptates excepturi deleniti ratione')

WS.verifyElementPropertyValue(response, '[3].postId', 1)
WS.verifyElementPropertyValue(response, '[3].id', 4)
WS.verifyElementPropertyValue(response, '[3].name', 'alias odio sit')
WS.verifyElementPropertyValue(response, '[3].email', 'Lew@alysha.tv')
WS.verifyElementPropertyValue(response, '[3].body', 'non et atque\noccaecati deserunt quas accusantium unde odit nobis qui voluptatem\nquia voluptas consequuntur itaque dolor\net qui rerum deleniti ut occaecati')

WS.verifyElementPropertyValue(response, '[4].postId', 1)
WS.verifyElementPropertyValue(response, '[4].id', 5)
WS.verifyElementPropertyValue(response, '[4].name', 'vero eaque aliquid doloribus et culpa')
WS.verifyElementPropertyValue(response, '[4].email', 'Hayden@althea.biz')
WS.verifyElementPropertyValue(response, '[4].body', 'harum non quasi et ratione\ntempore iure ex voluptates in ratione\nharum architecto fugit inventore cupiditate\nvoluptates magni quo et')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
