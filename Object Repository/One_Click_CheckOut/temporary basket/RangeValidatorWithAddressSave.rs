<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>RangeValidatorWithAddressSave</name>
   <tag></tag>
   <elementGuidId>015c0d7b-f04b-4407-a5b9-f434309b9b8a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/s/${SITEID}/dw/shop/v23_1/custom_objects/temporary_basket/rangeValidator?storeID=2608&amp;longitude=-99.1989004&amp;latitude=19.3345463&amp;product_id=T10158397&amp;crmAddressId=${crm_address_id}&amp;basketId=${temporary_basket_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.URL</defaultValue>
      <description></description>
      <id>b70aa738-65c1-4a75-80dd-4b525b324a8a</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SITEID</defaultValue>
      <description></description>
      <id>5999a306-41f6-46f1-bbca-e81f33951524</id>
      <masked>false</masked>
      <name>SITEID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.crm_address_id</defaultValue>
      <description></description>
      <id>0d35e973-a9af-424d-9202-06dda63c7cf9</id>
      <masked>false</masked>
      <name>crm_address_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.temporary_basket_id</defaultValue>
      <description></description>
      <id>ba304bc2-e266-48ad-b4d3-01c186807b33</id>
      <masked>false</masked>
      <name>temporary_basket_id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
