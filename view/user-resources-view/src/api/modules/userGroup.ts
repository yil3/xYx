/**
* @Author xYx
* @Date 2022-11-27 21:16:37
*/

import http from '@/api'

export const saveUserGroup = (data: Object) => {
  return http.post('/user_resource/user_group/save', data);
}

export const deleteUserGroup = (id: String) => {
  return http.delete(`/user_resource/user_group/delete?id=${id}`);
}

export const fetchUsergroupPage = (params: any) => {
  return http.get('/user_resource/user_group/page', params);
}

