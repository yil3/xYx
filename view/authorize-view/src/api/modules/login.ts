import http from '@/api'

export const login = (data: any) => {
  return http.post('/auth/authorize/token', data)
}

export const logout = () => {
  return http.post('/auth/authorize/logout')
}

export const refreshToken = (refreshToken: any) => {
  return http.post(`/auth/authorize/refresh/${refreshToken}`);
}

export const register = (data: any) => {
  return http.post('/users/user/register', data)
}
