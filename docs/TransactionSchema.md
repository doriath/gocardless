# TransactionSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> | transactionId | [optional]
**entry_reference** | Option<**String**> | entryReference | [optional]
**end_to_end_id** | Option<**String**> | endToEndId | [optional]
**mandate_id** | Option<**String**> | mandateId | [optional]
**check_id** | Option<**String**> | checkId | [optional]
**creditor_id** | Option<**String**> | creditorId | [optional]
**booking_date** | Option<**String**> | bookingDate | [optional]
**value_date** | Option<**String**> | valueDate | [optional]
**booking_date_time** | Option<**String**> | bookingDateTime | [optional]
**value_date_time** | Option<**String**> | valueDateTime | [optional]
**transaction_amount** | [**crate::models::TransactionAmountSchema**](TransactionAmountSchema.md) |  | 
**currency_exchange** | Option<[**Vec<crate::models::CurrencyExchangeSchema>**](CurrencyExchangeSchema.md)> |  | [optional]
**creditor_name** | Option<**String**> | creditorName | [optional]
**creditor_account** | Option<[**crate::models::AccountSchema**](AccountSchema.md)> |  | [optional]
**ultimate_creditor** | Option<**String**> | ultimateCreditor | [optional]
**debtor_name** | Option<**String**> | debtorName | [optional]
**debtor_account** | Option<[**crate::models::AccountSchema**](AccountSchema.md)> |  | [optional]
**ultimate_debtor** | Option<**String**> | ultimateDebtor | [optional]
**remittance_information_unstructured** | Option<**String**> | remittanceInformationUnstructured | [optional]
**remittance_information_unstructured_array** | Option<**Vec<String>**> | remittanceInformationUnstructuredArray | [optional]
**remittance_information_structured** | Option<**String**> | remittanceInformationStructured | [optional]
**remittance_information_structured_array** | Option<**Vec<String>**> | remittanceInformationStructuredArray | [optional]
**additional_information** | Option<**String**> | additionalInformation | [optional]
**purpose_code** | Option<**String**> | purposeCode | [optional]
**bank_transaction_code** | Option<**String**> | bankTransactionCode | [optional]
**proprietary_bank_transaction_code** | Option<**String**> | proprietaryBankTransactionCode | [optional]
**internal_transaction_id** | Option<**String**> | internalTransactionId | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


