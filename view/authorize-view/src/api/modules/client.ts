import http from '@/api'

/**
* @Author xYx
* @Date 2022-11-04 16:02:07
*/

export const saveClient = (data: any) => {
  return http.post('/auth/client/save_client', data);
};

export const fetchClientPage = (params: any) => {
  return http.get('/auth/client/page', params);
}

export const deleteClient = (id: any) => {
  return http.delete(`/auth/client/delete/${id}`);
}
