# \AccountsApi

All URIs are relative to *https://ob.gocardless.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_account_balances**](AccountsApi.md#retrieve_account_balances) | **GET** /api/v2/accounts/{id}/balances/ | 
[**retrieve_account_details**](AccountsApi.md#retrieve_account_details) | **GET** /api/v2/accounts/{id}/details/ | 
[**retrieve_account_metadata**](AccountsApi.md#retrieve_account_metadata) | **GET** /api/v2/accounts/{id}/ | 
[**retrieve_account_transactions**](AccountsApi.md#retrieve_account_transactions) | **GET** /api/v2/accounts/{id}/transactions/ | 



## retrieve_account_balances

> crate::models::AccountBalanceSchema retrieve_account_balances(id)


Access account balances.  Balances will be returned in Berlin Group PSD2 format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::AccountBalanceSchema**](AccountBalanceSchema.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_account_details

> crate::models::AccountDetailSchema retrieve_account_details(id)


Access account details.  Account details will be returned in Berlin Group PSD2 format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::AccountDetailSchema**](AccountDetailSchema.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_account_metadata

> crate::models::Account retrieve_account_metadata(id)


Access account metadata.  Information about the account record, such as the processing status and IBAN.  Account status is recalculated based on the error count in the latest req.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_account_transactions

> crate::models::BankTransactionStatusSchema retrieve_account_transactions(id, date_from, date_to)


Access account transactions.  Transactions will be returned in Berlin Group PSD2 format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |

### Return type

[**crate::models::BankTransactionStatusSchema**](BankTransactionStatusSchema.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

