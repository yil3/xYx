import http from '@/api'
/**
* @Author xYx
* @Date 2022-11-26 17:21:14
*/
export const fetchUserPage = (params: any) => {
  return http.get('/user_resource/user/page', params);
}
