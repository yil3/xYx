import NProgress from "@/config/nprogress";
import axios, { AxiosInstance, AxiosError, AxiosRequestConfig, AxiosResponse, Canceler } from "axios";
import { showFullScreenLoading, tryHideFullScreenLoading } from "@/config/serviceLoading";
// import { setToken } from "@/redux/modules/global/action";
import { message } from "antd";
// import { store } from "@/redux";

import { isFunction } from "@/utils/is/index";
import qs from "qs";
import { ResultData } from "./interface";

// * 声明一个 Map 用于存储每个请求的标识 和 取消函数
let pendingMap = new Map<string, Canceler>();

// * 序列化参数
export const getPendingUrl = (config: AxiosRequestConfig) => {
  return [config.method, config.url, qs.stringify(config.data), qs.stringify(config.params)].join("&");
};

export class AxiosCanceler {
  /**
   * @description: 添加请求
   * @param {Object} config
   */
  addPending(config: AxiosRequestConfig) {
    // * 在请求开始前，对之前的请求做检查取消操作
    this.removePending(config);
    const url = getPendingUrl(config);
    config.cancelToken =
      config.cancelToken ||
      new axios.CancelToken(cancel => {
        if (!pendingMap.has(url)) {
          // 如果 pending 中不存在当前请求，则添加进去
          pendingMap.set(url, cancel);
        }
      });
  }

  /**
   * @description: 移除请求
   * @param {Object} config
   */
  removePending(config: AxiosRequestConfig) {
    const url = getPendingUrl(config);

    if (pendingMap.has(url)) {
      // 如果在 pending 中存在当前请求标识，需要取消当前请求，并且移除
      const cancel = pendingMap.get(url);
      cancel && cancel();
      pendingMap.delete(url);
    }
  }

  /**
   * @description: 清空所有pending
   */
  removeAllPending() {
    pendingMap.forEach(cancel => {
      cancel && isFunction(cancel) && cancel();
    });
    pendingMap.clear();
  }

  /**
   * @description: 重置
   */
  reset(): void {
    pendingMap = new Map<string, Canceler>();
  }
}

/**
 * @description: 校验网络请求状态码
 * @param {Number} status
 * @return void
 */
export const checkStatus = (status: number): void => {
  switch (status) {
    case 400:
      message.error("请求失败！请您稍后重试");
      break;
    case 401:
      message.error("登录失效！请您重新登录");
      break;
    case 403:
      message.error("当前账号无权限访问！");
      break;
    case 404:
      message.error("你所访问的资源不存在！");
      break;
    case 405:
      message.error("请求方式错误！请您稍后重试");
      break;
    case 408:
      message.error("请求超时！请您稍后重试");
      break;
    case 500:
      message.error("服务异常！");
      break;
    case 502:
      message.error("网关错误！");
      break;
    case 503:
      message.error("服务不可用！");
      break;
    case 504:
      message.error("网关超时！");
      break;
    default:
      message.error("请求失败！");
  }
};

const axiosCanceler = new AxiosCanceler();

const config = {
  // 默认地址请求地址，可在 .env 开头文件中修改
  baseURL: import.meta.env.VITE_API_URL as string,
  // 设置超时时间（10s）
  timeout: 10000,
  // 跨域时候允许携带凭证
  withCredentials: true
};

class RequestHttp {
  service: AxiosInstance;
  public constructor(config: AxiosRequestConfig) {
    // 实例化axios
    this.service = axios.create(config);

    /**
     * @description 请求拦截器
     * 客户端发送请求 -> [请求拦截器] -> 服务器
     * token校验(JWT) : 接受服务器返回的token,存储到redux/本地储存当中
     */
    this.service.interceptors.request.use(
      (config: any) => {
        NProgress.start();
        // * 将当前请求添加到 pending 中
        axiosCanceler.addPending(config);
        // * 如果当前请求需要显示loading, 在api服务中通过指定的第三个参数: { loading: true } 来控制显示loading
        !config.loading || showFullScreenLoading();
        // const token: string = store.getState().global.token;
        const token = JSON.parse(localStorage.getItem("token") || "{}").accessToken;
        return { ...config, headers: { ...config.headers, "Authorization": "Bearer " + token } };
      },
      (error: AxiosError) => {
        return Promise.reject(error);
      }
    );

    /**
     * @description 响应拦截器
     *  服务器换返回信息 -> [拦截统一处理] -> 客户端JS获取到信息
     */
    this.service.interceptors.response.use(
      (response: AxiosResponse) => {
        const { data, config } = response;
        NProgress.done();
        // * 在请求结束后，移除本次请求(关闭loading)
        axiosCanceler.removePending(config);
        tryHideFullScreenLoading();
        if (data.success === false) {
          if (data.msg) {
            message.error(data.msg);
          }
          return Promise.reject(data);
        }
        // * 成功请求（在页面上除非特殊情况，否则不用处理失败逻辑）
        // return Promise.resolve(data);
        return data;
      },
      async (error: AxiosError) => {
        const { response } = error;
        NProgress.done();
        tryHideFullScreenLoading();
        // 请求超时单独判断，请求超时没有 response
        if (error.message.indexOf("timeout") !== -1) message.error("请求超时，请稍后再试");
        // 根据响应的错误状态码，做不同的处理
        if (response) checkStatus(response.status);
        // * 登录过期，跳转到登录页
        if (response?.status == 401) {
          localStorage.removeItem("token");
          window.location.href = 'http://localhost:3000/authorize' +
            '?redirect_uri=http://localhost:3010' +
            '&response_type=code' +
            '&client_id=00000000-0000-0000-0000-000000000001' +
            '&client_secret=aa332211 ' +
            '&scope=all&state=x';
        }
        // 服务器结果都没有返回(可能服务器错误可能客户端断网) 断网处理:可以跳转到断网页面
        if (!window.navigator.onLine) window.location.href = "/500";
        // return Promise.reject(error);
        return error;
      }
    );
  }

  // * 常用请求方法封装
  async get(url: string, params?: object, _object = {}): Promise<ResultData> {
    return this.service.get(url, { params, ..._object });
  }
  async post(url: string, data?: object, _object = {}): Promise<ResultData> {
    return this.service.post(url, data, _object);
  }
  async put(url: string, data?: object, _object = {}): Promise<ResultData> {
    return this.service.put(url, data, _object);
  }
  async delete(url: string, params?: any, _object = {}): Promise<ResultData> {
    return this.service.delete(url, { params, ..._object });
  }
}

export default new RequestHttp(config);
