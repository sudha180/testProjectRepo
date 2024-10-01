<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PlaceOrder</name>
   <tag></tag>
   <elementGuidId>c7d78b00-6fc4-4c9a-9ec6-38574086a47c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ2ZXIiOiIxLjAiLCJqa3UiOiJzbGFzL3Byb2QvYmpjbF9kZXYiLCJraWQiOiJmMDQ0YjJjMi1iNWYwLTQwNWMtOGU2OS02ZGFiZDY3Mjg5NzEiLCJ0eXAiOiJqd3QiLCJjbHYiOiJKMi4zLjQiLCJhbGciOiJFUzI1NiJ9.eyJhdXQiOiJHVUlEIiwic2NwIjoic2ZjYy5zaG9wcGVyLW15YWNjb3VudC5iYXNrZXRzIHNmY2Muc2hvcHBlci1kaXNjb3Zlcnktc2VhcmNoIHNmY2Muc2hvcHBlci1teWFjY291bnQuYWRkcmVzc2VzIHNmY2Muc2hvcHBlci1wcm9kdWN0cyBzZmNjLnNob3BwZXItbXlhY2NvdW50LnJ3IHNmY2Muc2hvcHBlci1teWFjY291bnQucGF5bWVudGluc3RydW1lbnRzIHNmY2Muc2hvcHBlci1jdXN0b21lcnMubG9naW4gc2ZjYy5zaG9wcGVyLXN0b3JlcyBzZmNjLnNob3BwZXItbXlhY2NvdW50Lm9yZGVycyBzZmNjLnNob3BwZXItYmFza2V0cy1vcmRlcnMgc2ZjYy5zaG9wcGVyLWN1c3RvbWVycy5yZWdpc3RlciBzZmNjLnNob3BwZXItbXlhY2NvdW50LmFkZHJlc3Nlcy5ydyBzZmNjLnNob3BwZXItbXlhY2NvdW50LnByb2R1Y3RsaXN0cy5ydyBzZmNjLnNob3BwZXItcHJvZHVjdGxpc3RzIHNmY2Muc2hvcHBlci1wcm9tb3Rpb25zIHNmY2Muc2hvcHBlci1iYXNrZXRzLW9yZGVycy5ydyBzZmNjLnNob3BwZXItZ2lmdC1jZXJ0aWZpY2F0ZXMgc2ZjYy5zaG9wcGVyLW15YWNjb3VudC5wYXltZW50aW5zdHJ1bWVudHMucncgc2ZjYy5zaG9wcGVyLXByb2R1Y3Qtc2VhcmNoIHNmY2Muc2hvcHBlci1teWFjY291bnQucHJvZHVjdGxpc3RzIHNmY2Muc2hvcHBlci1jYXRlZ29yaWVzIHNmY2Muc2hvcHBlci1teWFjY291bnQiLCJzdWIiOiJjYy1zbGFzOjpiamNsX2Rldjo6c2NpZDo1ZDI1ZjVmYS01YTRmLTQ2YjUtYTIxMS04NzU1NDJjODVhYzE6OnVzaWQ6MjY1MzYxMWUtMGE5Yi00NWU4LTlkMjItMDI5ZDEwYzBhY2ZhIiwiY3R4Ijoic2xhcyIsImlzcyI6InNsYXMvcHJvZC9iamNsX2RldiIsImlzdCI6MSwiZG50IjoiMCIsImF1ZCI6ImNvbW1lcmNlY2xvdWQvcHJvZC9iamNsX2RldiIsIm5iZiI6MTcxMTk5NjkwMiwic3R5IjoiVXNlciIsImlzYiI6InVpZG86ZWNvbTo6dXBuOnFhLnRvdGFscGxheSswMTAwOTg3MTA1QGdtYWlsLmNvbTo6dWlkbjpSaWNhcmRvIENvcm9uYSBDcnV6OjpnY2lkOmFia0h3WmtYd1ZrdXNSa2VjM3dHWVlsdXMyOjpyY2lkOmFjblJiV3F1akhDcGc5b3E2VjJoa3VUYVNXOjpjaGlkOlRQLUZPT0QiLCJleHAiOjE3MTE5OTg3MzIsImlhdCI6MTcxMTk5NjkzMiwianRpIjoiQzJDLTEzMTE1NjM1NzAxNzg3NTExODUwMzMyNDI2NjU1OTI2NjE2NyJ9.Lvy2CDcttFTnqtgxuwa6xzWD7bureCj8Gt_DuCTDQN1zzny82dyD5clOt6ZUGQzPl4SpviMAGB8chXRxsGAtug</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;basket_id\&quot; : \&quot;${GlobalVariable.FoodBasketId}\&quot;\n}&quot;,
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
      <webElementGuid>5b920d27-a1cf-4e4d-a849-7ccf4db78b51</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.Authorization}</value>
      <webElementGuid>b86d5919-cd4b-4296-a659-3a75ffbee378</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.URL}/s/TP-FOOD/dw/shop/v24_5/orders</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.URL</defaultValue>
      <description></description>
      <id>a484f435-e4bc-42ba-991e-85b670bbdb33</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.shop_auth_token</defaultValue>
      <description></description>
      <id>f13d208e-3805-4dd0-bd6b-5d4356aebcf2</id>
      <masked>false</masked>
      <name>shop_auth_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FoodBasketId</defaultValue>
      <description></description>
      <id>06621fb8-f70b-4e23-ab13-343fa648e7db</id>
      <masked>false</masked>
      <name>basketID</name>
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

WS.verifyElementPropertyValue(response, '_v', &quot;24.5&quot;)
WS.verifyElementPropertyValue(response, '_type', &quot;order&quot;)
WS.verifyElementPropertyValue(response, '_resource_state', &quot;dac0fb96b4781cff9a43505923f1a394a427148a9216993228580acb70780c70&quot;)
WS.verifyElementPropertyValue(response, 'adjusted_merchandize_total_tax', 0.00)
WS.verifyElementPropertyValue(response, 'adjusted_shipping_total_tax', 7.36)
WS.verifyElementPropertyValue(response, 'billing_address._type', &quot;order_address&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.address1', &quot;Alejandro Dumas&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.city', &quot;Ciudad de Mexico&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.country_code', &quot;MX&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.first_name', &quot;CESAR&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.full_name', &quot;CESAR GERARDO WALDO GONZALEZ&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.id', &quot;6f3c9518cf71a372c6d0d08798&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.last_name', &quot;GERARDO WALDO GONZALEZ&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.phone', &quot;5579660270&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.postal_code', &quot;11500&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_alias', &quot;CasaWaldoP&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_delegation', &quot;Miguel Hidalgo&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_label', &quot;{\&quot;home\&quot;:true,\&quot;company\&quot;:false,\&quot;other\&quot;:false}&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_latitude', &quot;19.439691&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_longitude', &quot;-99.205670&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_nExt', &quot;107&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_neighborhood', &quot;Polanco&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_references', &quot;Llamar al llegar&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_rfc', &quot;XAXA010101111&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_state', &quot;CDMX&quot;)
WS.verifyElementPropertyValue(response, 'billing_address.c_streetName', &quot;Alejandro Dumas&quot;)
WS.verifyElementPropertyValue(response, 'channel_type', &quot;storefront&quot;)
WS.verifyElementPropertyValue(response, 'confirmation_status', &quot;confirmed&quot;)
WS.verifyElementPropertyValue(response, 'created_by', &quot;Customer&quot;)
WS.verifyElementPropertyValue(response, 'creation_date', &quot;2024-09-04T05:07:29.656Z&quot;)
WS.verifyElementPropertyValue(response, 'currency', &quot;MXN&quot;)
WS.verifyElementPropertyValue(response, 'customer_info._type', &quot;customer_info&quot;)
WS.verifyElementPropertyValue(response, 'customer_info.customer_id', &quot;acTAcwrTj6ECo7V8WDIa5leqor&quot;)
WS.verifyElementPropertyValue(response, 'customer_info.customer_name', &quot;CESAR GERARDO WALDO GONZALEZ&quot;)
WS.verifyElementPropertyValue(response, 'customer_info.customer_no', &quot;00012002&quot;)
WS.verifyElementPropertyValue(response, 'customer_info.email', &quot;cesar.waldo93@gmail.com&quot;)
WS.verifyElementPropertyValue(response, 'customer_name', &quot;CESAR GERARDO WALDO GONZALEZ&quot;)
WS.verifyElementPropertyValue(response, 'export_status', &quot;ready&quot;)
WS.verifyElementPropertyValue(response, 'grouped_tax_items[0]._type', &quot;grouped_tax&quot;)
WS.verifyElementPropertyValue(response, 'grouped_tax_items[0].tax_rate', 0.16)
WS.verifyElementPropertyValue(response, 'grouped_tax_items[0].tax_value', 7.36)
WS.verifyElementPropertyValue(response, 'guest', false)
WS.verifyElementPropertyValue(response, 'last_modified', &quot;2024-09-04T05:07:30.152Z&quot;)
WS.verifyElementPropertyValue(response, 'merchandize_total_tax', 0.00)
WS.verifyElementPropertyValue(response, 'notes._type', &quot;simple_link&quot;)
WS.verifyElementPropertyValue(response, 'notes.link', &quot;https://salesforceqa.sistemastp.com.mx/s/TP-FOOD/dw/shop/v24_5/orders/00021306/notes&quot;)
WS.verifyElementPropertyValue(response, 'order_no', &quot;00021306&quot;)
WS.verifyElementPropertyValue(response, 'order_token', &quot;HuIA0Tt1pRwwV04SV_-K9xUgciOzjrfmcxyOPCkfDdg&quot;)
WS.verifyElementPropertyValue(response, 'order_total', 85.36)
WS.verifyElementPropertyValue(response, 'payment_status', &quot;not_paid&quot;)
WS.verifyElementPropertyValue(response, 'product_sub_total', 32.00)
WS.verifyElementPropertyValue(response, 'product_total', 32.00)
WS.verifyElementPropertyValue(response, 'shipments[0]._type', &quot;shipment&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].adjusted_merchandize_total_tax', 0.00)
WS.verifyElementPropertyValue(response, 'shipments[0].adjusted_shipping_total_tax', 7.36)
WS.verifyElementPropertyValue(response, 'shipments[0].gift', false)
WS.verifyElementPropertyValue(response, 'shipments[0].merchandize_total_tax', 0.00)
WS.verifyElementPropertyValue(response, 'shipments[0].product_sub_total', 32.00)
WS.verifyElementPropertyValue(response, 'shipments[0].product_total', 32.00)
WS.verifyElementPropertyValue(response, 'shipments[0].shipment_id', &quot;00021306-A&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipment_no', &quot;00019230&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipment_total', 85.36)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address._type', &quot;order_address&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.address1', &quot;Alejandro Dumas&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.city', &quot;Ciudad de Mexico&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.country_code', &quot;MX&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.first_name', &quot;CESAR&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.full_name', &quot;CESAR GERARDO WALDO GONZALEZ&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.id', &quot;adbbc6b3506830fb24682fe107&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.last_name', &quot;GERARDO WALDO GONZALEZ&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.phone', &quot;5579660270&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.postal_code', &quot;11500&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_alias', &quot;CasaWaldoP&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_delegation', &quot;Miguel Hidalgo&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_label', &quot;{\&quot;home\&quot;:true,\&quot;company\&quot;:false,\&quot;other\&quot;:false}&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_latitude', &quot;19.439691&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_longitude', &quot;-99.205670&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_nExt', &quot;107&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_neighborhood', &quot;Polanco&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_references', &quot;Llamar al llegar&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_rfc', &quot;XAXA010101111&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_state', &quot;CDMX&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_streetName', &quot;Alejandro Dumas&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_etiqueta.home', true)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_etiqueta.company', false)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_address.c_etiqueta.other', false)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_method._type', &quot;shipping_method&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_method.description', &quot;pickup&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_method.id', &quot;pickup&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_method.name', &quot;pickup&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_method.c_estimatedArrivalTime', &quot;60 Mins&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_method.c_storePickupEnabled', false)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_status', &quot;not_shipped&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_total', 46.00)
WS.verifyElementPropertyValue(response, 'shipments[0].shipping_total_tax', 7.36)

WS.verifyElementPropertyValue(response, 'shipments[0].tax_total', 7.36)
WS.verifyElementPropertyValue(response, 'shipments[0].c_canCancel', true)
WS.verifyElementPropertyValue(response, 'shipments[0].c_canEvaluate', false)
WS.verifyElementPropertyValue(response, 'shipments[0].c_hasIncident', false)
WS.verifyElementPropertyValue(response, 'shipments[0].c_hasInvoice', false)
WS.verifyElementPropertyValue(response, 'shipments[0].c_miraklOrderId', &quot;00021306-A&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_miraklOrderStatus', &quot;WAITING_ACCEPTANCE&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_miraklShippingLabel', &quot;Picker Express&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_serviceCharge', 0.0)
WS.verifyElementPropertyValue(response, 'shipments[0].c_shippingRates', &quot;[\&quot;{\\\&quot;shipCost\\\&quot;:40,\\\&quot;idCotizacion\\\&quot;:\\\&quot;PICK000000011717\\\&quot;}\&quot;]&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_shopId', &quot;4158&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_shopName', &quot;La Casa De Toño Polanco&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_totalShippingCost', 53.36)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0]._type', &quot;product_item&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].adjusted_tax', 0.00)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].base_price', 32.00)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].bonus_product_line_item', false)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].gift', false)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].inventory_id', &quot;inventory_m_food_mirakl&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].item_id', &quot;733d4be25f9c47af0e3b36dcab&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].item_text', &quot;Orden de Maciza (100 g)&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].price', 32.00)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].price_after_item_discount', 32.00)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].price_after_order_discount', 32.00)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].product_id', &quot;C828554986&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].product_name', &quot;Orden de Maciza (100 g)&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].quantity', 1)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].shipment_id', &quot;me&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].tax', 0.00)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].tax_basis', 32.00)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].tax_class_id', &quot;product_zero_tax&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].tax_rate', 0.0)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_canOpenIncident', false)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_fromStoreId', &quot;4158&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_isOnDemandStoreItem', true)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_mainLineItemId', &quot;7b944bd54930d7375f267fc87a&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_miraklChannels', &quot;[\&quot;INIT\&quot;]&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_miraklEarliestDeliveryDays', 1.0)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_miraklLatestDeliveryDays', 2.0)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_miraklLeadTimeToShip', &quot;2&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_miraklLineShippingPrice', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_miraklOperatorProductId', &quot;C828554986&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_miraklShippingTaxes', &quot;[{\&quot;amount\&quot;:0,\&quot;type\&quot;:\&quot;product_zero_tax\&quot;,\&quot;isTaxRate\&quot;:true,\&quot;rate\&quot;:0}]&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_noOfPersons', 1)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_offerId', &quot;541185&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_offerSalesPrice', 32.0)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_offerState', &quot;Nuevo&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_offerStateCode', &quot;11&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_orderLineId', &quot;00021306-A-1&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_shopId', &quot;4158&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_shopName', &quot;La Casa De Toño Polanco&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_total_crossed_price', 32.0)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_priceWithComplement', 32.0)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_imagesData.large[0].alt', &quot;Orden de Maciza (100 g)&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_imagesData.large[0].url', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_productItems[0].c_imagesData.large[0].title', &quot;Orden de Maciza (100 g)&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0]._type', &quot;order_payment_instrument&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].amount', 85.36)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].payment_card._type', &quot;payment_card&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].payment_card.card_type', &quot;Visa&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].payment_card.credit_card_expired', true)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].payment_card.expiration_month', 6)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].payment_card.expiration_year', 2024)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].payment_card.holder', &quot;Test Dev&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].payment_instrument_id', &quot;de173a6158ad521df77a3f0bef&quot;)
WS.verifyElementPropertyValue(response, 'shipments[0].c_paymentInstruments[0].payment_method_id', &quot;CREDIT_CARD&quot;)
WS.verifyElementPropertyValue(response, 'shipping_items[0]._type', &quot;shipping_item&quot;)
WS.verifyElementPropertyValue(response, 'shipping_items[0].adjusted_tax', 7.36)
WS.verifyElementPropertyValue(response, 'shipping_items[0].base_price', 46.00)
WS.verifyElementPropertyValue(response, 'shipping_items[0].item_id', &quot;1a419eafc9d2d073d2f135d1aa&quot;)
WS.verifyElementPropertyValue(response, 'shipping_items[0].item_text', &quot;Shipping&quot;)
WS.verifyElementPropertyValue(response, 'shipping_items[0].price', 46.00)
WS.verifyElementPropertyValue(response, 'shipping_items[0].price_after_item_discount', 46.00)
WS.verifyElementPropertyValue(response, 'shipping_items[0].shipment_id', &quot;me&quot;)
WS.verifyElementPropertyValue(response, 'shipping_items[0].tax', 7.36)
WS.verifyElementPropertyValue(response, 'shipping_items[0].tax_basis', 46.00)
WS.verifyElementPropertyValue(response, 'shipping_items[0].tax_class_id', &quot;new_tax&quot;)
WS.verifyElementPropertyValue(response, 'shipping_items[0].tax_rate', 0.16)
WS.verifyElementPropertyValue(response, 'shipping_status', &quot;not_shipped&quot;)
WS.verifyElementPropertyValue(response, 'shipping_total', 46.00)
WS.verifyElementPropertyValue(response, 'shipping_total_tax', 7.36)
WS.verifyElementPropertyValue(response, 'site_id', &quot;TP-FOOD&quot;)
WS.verifyElementPropertyValue(response, 'status', &quot;new&quot;)
WS.verifyElementPropertyValue(response, 'taxation', &quot;net&quot;)
WS.verifyElementPropertyValue(response, 'tax_rounded_at_group', false)
WS.verifyElementPropertyValue(response, 'tax_total', 7.36)
WS.verifyElementPropertyValue(response, 'c_currentCorporate', &quot;LaCasaDeToño&quot;)
WS.verifyElementPropertyValue(response, 'c_isMiraklOrder', true)
WS.verifyElementPropertyValue(response, 'c_miraklCustomerId', &quot;0106378356&quot;)
WS.verifyElementPropertyValue(response, 'c_miraklFraudCheckStatus', &quot;OK&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
