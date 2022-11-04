import http from '@/api'

/**
* @Author xYx
* @Date 2022-11-04 16:02:07
*/

export const saveClient = (data: any) => {
  return http.post('/auth/client/save_client', data);
};

export const fetchClientList = (params: any) => {
  return http.get('/auth/client/list', {params});
}
