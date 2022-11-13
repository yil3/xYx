import http from '@/api';


export const fetchTokenPage = (params: any) => {
  return http.get('/auth/token/page', params);
}

export const deleteToken = (id: any) => {
  return http.delete(`/auth/token/delete/${id}`);
}

