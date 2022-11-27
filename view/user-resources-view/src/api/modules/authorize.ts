import http from '@/api'

export const token = (data: any) => {
  return http.post('/authen/authorize/token', data)
}
