import http from '@/api'

export const token = (data: any) => {
  return http.post('/authen/authorize/token', data)
}

export const logout = () => {
  return http.post('/authen/authorize/logout')
}

export const refreshToken = (refreshToken: any) => {
  return http.post(`/authen/authorize/refresh/${refreshToken}`);
}

export const register = (data: any) => {
  return http.post('/user_resource/user/register', data)
}

export const authorize = (params: any) => {
  return http.get('/authen/authorize', params)
}
