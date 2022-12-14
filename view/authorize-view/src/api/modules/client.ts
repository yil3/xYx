import http from '@/api'

/**
* @Author xYx
* @Date 2022-11-04 16:02:07
*/

export const saveClient = (data: any) => {
  return http.post('/authen/client/save_client', data);
};

export const fetchClientPage = (params: any) => {
  return http.get('/authen/client/page', params);
}

export const deleteClient = (id: any) => {
  return http.delete(`/authen/client/delete/${id}`);
}
