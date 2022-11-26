import http from '@/api';


export const fetchTokenPage = (params: any) => {
  return http.get('/authen/token/page', params);
}

export const deleteToken = (id: any) => {
  return http.delete(`/authen/token/delete/${id}`);
}

