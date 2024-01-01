# PaymentWriteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**institution_id** | **String** | Institution ID for Payment | 
**payment_product** | Option<[**crate::models::PaymentProductEnum**](PaymentProductEnum.md)> |  | [optional][default to Isct]
**instructed_amount** | [**crate::models::InstructedAmountRequest**](InstructedAmountRequest.md) |  | 
**creditor_object** | [**crate::models::CreditorAccountWriteRequest**](CreditorAccountWriteRequest.md) |  | 
**debtor_account** | Option<[**crate::models::DebtorAccountWriteRequest**](DebtorAccountWriteRequest.md)> |  | [optional]
**redirect** | Option<**String**> | Redirect URL to your application after payment is done | 
**description** | Option<**String**> | Payment description | [optional][default to GOCARDLESS]
**custom_payment_id** | Option<**String**> | Payment Custom Payment ID | [optional]
**requested_execution_date** | Option<[**String**](string.md)> | Payment Execution date (for periodic payments) | [optional]
**periodic_payment** | Option<[**crate::models::PeriodicPaymentRequest**](PeriodicPaymentRequest.md)> |  | [optional]
**submit_payment** | Option<**bool**> | Indicates whether payment should be submitted separately | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


