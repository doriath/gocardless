# Bacs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**link** | Option<**String**> | Link to initiate authorization with Institution | [optional][readonly][default to https://ob.nordigen.com/pis/start/0287b538-a4ee-46f4-8117-5d2c6b793ce3/{$INSTITUTION_ID}]
**payment_id** | Option<**String**> | Payment ID | [optional][readonly]
**payment_status** | Option<[**crate::models::PaymentStatusEnum**](PaymentStatusEnum.md)> |  | [optional][readonly]
**payment_product** | Option<[**crate::models::PaymentProductEnum**](PaymentProductEnum.md)> |  | [optional][default to Isct]
**payment_type** | Option<[**crate::models::PaymentTypeEnum**](PaymentTypeEnum.md)> |  | [optional][readonly]
**instructed_amount** | [**crate::models::InstructedAmount**](InstructedAmount.md) |  | 
**creditor_object** | [**crate::models::CreditorAccountWrite**](CreditorAccountWrite.md) |  | 
**redirect** | Option<**String**> | Redirect URL to your application after payment is done | 
**description** | Option<**String**> | Payment description | [optional][default to GOCARDLESS]
**custom_payment_id** | Option<**String**> | Payment Custom Payment ID | [optional]
**requested_execution_date** | Option<[**String**](string.md)> | Payment Execution date (for periodic payments) | [optional]
**submit_payment** | Option<**bool**> | Indicates whether payment should be submitted separately | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


