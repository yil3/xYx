import http from '@/api';
/**
* @Author xYx
* @Date 2022-12-01 16:08:16
*/

export const getRolePage = (params: any) => {
  return http.get('user_resource/role/page', params);
};

export const saveRole = (data: any) => {
  return http.post('user_resource/role/save', data);
};

export const deleteRole = (id: String) => {
  return http.delete('user_resource/role/delete', { id });
}

export const getRoleTree = (params: any) => {
  return http.get('user_resource/role/tree', params);
};
