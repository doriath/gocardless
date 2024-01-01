# \TokenApi

All URIs are relative to *https://ob.gocardless.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_a_new_access_token**](TokenApi.md#get_a_new_access_token) | **POST** /api/v2/token/refresh/ | 
[**obtain_new_access_slash_refresh_token_pair**](TokenApi.md#obtain_new_access_slash_refresh_token_pair) | **POST** /api/v2/token/new/ | 



## get_a_new_access_token

> crate::models::SpectacularJwtRefresh get_a_new_access_token(jwt_refresh_request)


Refresh access token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jwt_refresh_request** | [**JwtRefreshRequest**](JwtRefreshRequest.md) |  | [required] |

### Return type

[**crate::models::SpectacularJwtRefresh**](SpectacularJWTRefresh.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## obtain_new_access_slash_refresh_token_pair

> crate::models::SpectacularJwtObtain obtain_new_access_slash_refresh_token_pair(jwt_obtain_pair_request)


Obtain JWT pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jwt_obtain_pair_request** | [**JwtObtainPairRequest**](JwtObtainPairRequest.md) |  | [required] |

### Return type

[**crate::models::SpectacularJwtObtain**](SpectacularJWTObtain.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

