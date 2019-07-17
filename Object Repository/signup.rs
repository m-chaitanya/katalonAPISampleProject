<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>signup</name>
   <tag></tag>
   <elementGuidId>ce6cf06e-8663-436c-a67c-0c2317fe6a43</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;username\&quot;: \&quot;${username}\&quot;,\n    \&quot;firstName\&quot;: \&quot;${firstName}\&quot;,\n    \&quot;lastName\&quot;: \&quot;{lastName}\&quot;,\n    \&quot;password\&quot;: \&quot;${password}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;mfaType\&quot;: \&quot;${mfaType}\&quot;\n}&quot;,
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
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://8jr2ucesma.execute-api.us-east-1.amazonaws.com/baron/public/signup</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'chaithu2'</defaultValue>
      <description></description>
      <id>b886987a-b0b2-447f-bbfa-601c47146afb</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'chaithanya'</defaultValue>
      <description></description>
      <id>a9a8eb74-222a-4e30-87b2-362e56dbd09f</id>
      <masked>false</masked>
      <name>firstName</name>
   </variables>
   <variables>
      <defaultValue>'mukk'</defaultValue>
      <description></description>
      <id>fe381e2b-6645-4cc2-8e28-2855c8d8899d</id>
      <masked>false</masked>
      <name>lastName</name>
   </variables>
   <variables>
      <defaultValue>'Password@123'</defaultValue>
      <description></description>
      <id>15c94349-00e9-4f15-81ba-c6536487be75</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <variables>
      <defaultValue>'chaithu.m@gmail.com'</defaultValue>
      <description></description>
      <id>3df581be-6377-4cc7-8863-0ff38dd03491</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'none'</defaultValue>
      <description></description>
      <id>c4ff97ba-bda1-4e3a-891d-1f3c769f2e5b</id>
      <masked>false</masked>
      <name>mfaType</name>
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



WS.verifyResponseStatusCode(response, 500)

assertThat(response.getStatusCode()).isEqualTo(500)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
