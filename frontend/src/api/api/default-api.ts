/* tslint:disable */
/* eslint-disable */
/**
 * 業務割当API
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import type { Configuration } from '../configuration';
import type { AxiosPromise, AxiosInstance, RawAxiosRequestConfig } from 'axios';
import globalAxios from 'axios';
// Some imports not used depending on template conditions
// @ts-ignore
import { DUMMY_BASE_URL, assertParamExists, setApiKeyToObject, setBasicAuthToObject, setBearerAuthToObject, setOAuthToObject, setSearchParams, serializeDataIfNeeded, toPathString, createRequestFunction } from '../common';
// @ts-ignore
import { BASE_PATH, COLLECTION_FORMATS, type RequestArgs, BaseAPI, RequiredError, operationServerMap } from '../base';
// @ts-ignore
import type { Task } from '../model';
// @ts-ignore
import type { TasksGetRequest } from '../model';
/**
 * DefaultApi - axios parameter creator
 * @export
 */
export const DefaultApiAxiosParamCreator = function (configuration?: Configuration) {
    return {
        /**
         * 
         * @summary サーバーのヘルスチェック
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        healthGet: async (options: RawAxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/health`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * 
         * @summary タスクの一覧を取得
         * @param {TasksGetRequest} [tasksGetRequest] 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        tasksGet: async (tasksGetRequest?: TasksGetRequest, options: RawAxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/tasks`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            localVarHeaderParameter['Content-Type'] = 'application/json';

            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};
            localVarRequestOptions.data = serializeDataIfNeeded(tasksGetRequest, localVarRequestOptions, configuration)

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
        /**
         * 
         * @summary タスクを新規作成
         * @param {Task} [task] 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        tasksPost: async (task?: Task, options: RawAxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/tasks`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, DUMMY_BASE_URL);
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }

            const localVarRequestOptions = { method: 'POST', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;


    
            localVarHeaderParameter['Content-Type'] = 'application/json';

            setSearchParams(localVarUrlObj, localVarQueryParameter);
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};
            localVarRequestOptions.data = serializeDataIfNeeded(task, localVarRequestOptions, configuration)

            return {
                url: toPathString(localVarUrlObj),
                options: localVarRequestOptions,
            };
        },
    }
};

/**
 * DefaultApi - functional programming interface
 * @export
 */
export const DefaultApiFp = function(configuration?: Configuration) {
    const localVarAxiosParamCreator = DefaultApiAxiosParamCreator(configuration)
    return {
        /**
         * 
         * @summary サーバーのヘルスチェック
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async healthGet(options?: RawAxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<string>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.healthGet(options);
            const localVarOperationServerIndex = configuration?.serverIndex ?? 0;
            const localVarOperationServerBasePath = operationServerMap['DefaultApi.healthGet']?.[localVarOperationServerIndex]?.url;
            return (axios, basePath) => createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration)(axios, localVarOperationServerBasePath || basePath);
        },
        /**
         * 
         * @summary タスクの一覧を取得
         * @param {TasksGetRequest} [tasksGetRequest] 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async tasksGet(tasksGetRequest?: TasksGetRequest, options?: RawAxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<Array<Task>>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.tasksGet(tasksGetRequest, options);
            const localVarOperationServerIndex = configuration?.serverIndex ?? 0;
            const localVarOperationServerBasePath = operationServerMap['DefaultApi.tasksGet']?.[localVarOperationServerIndex]?.url;
            return (axios, basePath) => createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration)(axios, localVarOperationServerBasePath || basePath);
        },
        /**
         * 
         * @summary タスクを新規作成
         * @param {Task} [task] 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async tasksPost(task?: Task, options?: RawAxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => AxiosPromise<Task>> {
            const localVarAxiosArgs = await localVarAxiosParamCreator.tasksPost(task, options);
            const localVarOperationServerIndex = configuration?.serverIndex ?? 0;
            const localVarOperationServerBasePath = operationServerMap['DefaultApi.tasksPost']?.[localVarOperationServerIndex]?.url;
            return (axios, basePath) => createRequestFunction(localVarAxiosArgs, globalAxios, BASE_PATH, configuration)(axios, localVarOperationServerBasePath || basePath);
        },
    }
};

/**
 * DefaultApi - factory interface
 * @export
 */
export const DefaultApiFactory = function (configuration?: Configuration, basePath?: string, axios?: AxiosInstance) {
    const localVarFp = DefaultApiFp(configuration)
    return {
        /**
         * 
         * @summary サーバーのヘルスチェック
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        healthGet(options?: RawAxiosRequestConfig): AxiosPromise<string> {
            return localVarFp.healthGet(options).then((request) => request(axios, basePath));
        },
        /**
         * 
         * @summary タスクの一覧を取得
         * @param {TasksGetRequest} [tasksGetRequest] 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        tasksGet(tasksGetRequest?: TasksGetRequest, options?: RawAxiosRequestConfig): AxiosPromise<Array<Task>> {
            return localVarFp.tasksGet(tasksGetRequest, options).then((request) => request(axios, basePath));
        },
        /**
         * 
         * @summary タスクを新規作成
         * @param {Task} [task] 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        tasksPost(task?: Task, options?: RawAxiosRequestConfig): AxiosPromise<Task> {
            return localVarFp.tasksPost(task, options).then((request) => request(axios, basePath));
        },
    };
};

/**
 * DefaultApi - interface
 * @export
 * @interface DefaultApi
 */
export interface DefaultApiInterface {
    /**
     * 
     * @summary サーバーのヘルスチェック
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApiInterface
     */
    healthGet(options?: RawAxiosRequestConfig): AxiosPromise<string>;

    /**
     * 
     * @summary タスクの一覧を取得
     * @param {TasksGetRequest} [tasksGetRequest] 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApiInterface
     */
    tasksGet(tasksGetRequest?: TasksGetRequest, options?: RawAxiosRequestConfig): AxiosPromise<Array<Task>>;

    /**
     * 
     * @summary タスクを新規作成
     * @param {Task} [task] 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApiInterface
     */
    tasksPost(task?: Task, options?: RawAxiosRequestConfig): AxiosPromise<Task>;

}

/**
 * DefaultApi - object-oriented interface
 * @export
 * @class DefaultApi
 * @extends {BaseAPI}
 */
export class DefaultApi extends BaseAPI implements DefaultApiInterface {
    /**
     * 
     * @summary サーバーのヘルスチェック
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApi
     */
    public healthGet(options?: RawAxiosRequestConfig) {
        return DefaultApiFp(this.configuration).healthGet(options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * 
     * @summary タスクの一覧を取得
     * @param {TasksGetRequest} [tasksGetRequest] 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApi
     */
    public tasksGet(tasksGetRequest?: TasksGetRequest, options?: RawAxiosRequestConfig) {
        return DefaultApiFp(this.configuration).tasksGet(tasksGetRequest, options).then((request) => request(this.axios, this.basePath));
    }

    /**
     * 
     * @summary タスクを新規作成
     * @param {Task} [task] 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof DefaultApi
     */
    public tasksPost(task?: Task, options?: RawAxiosRequestConfig) {
        return DefaultApiFp(this.configuration).tasksPost(task, options).then((request) => request(this.axios, this.basePath));
    }
}

