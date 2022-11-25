// * 请求响应参数(不包含data)
export interface Result {
	success: boolean;
	msg: string;
}

// * 请求响应参数(包含data)
export interface ResultData<T = any> extends Result {
	data?: T;
}
