# PaymentReadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**institution_id** | **String** | Institution ID for Payment | 
**payment_product** | Option<[**crate::models::PaymentProductEnum**](PaymentProductEnum.md)> |  | [optional][default to Isct]
**redirect** | Option<**String**> | Redirect URL to your application after payment is done | 
**description** | Option<**String**> | Payment description | [optional][default to GOCARDLESS]
**custom_payment_id** | Option<**String**> | Payment Custom Payment ID | [optional]
**creditor_object** | [**crate::models::CreditorAccountWriteRequest**](CreditorAccountWriteRequest.md) |  | 
**debtor_account** | [**crate::models::DebtorAccountWriteRequest**](DebtorAccountWriteRequest.md) |  | 
**instructed_amount** | [**crate::models::InstructedAmountRequest**](InstructedAmountRequest.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


