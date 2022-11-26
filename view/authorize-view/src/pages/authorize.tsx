import qs from 'qs';
import { authorize } from '@/api/modules/login';
import { Button, message } from 'antd';
import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
/**
* @Author xYx
* @Date 2022-11-25 17:16:41
*/
export default function Authorize() {
  const navigate = useNavigate();
  let params = qs.parse(location.search, { ignoreQueryPrefix: true });
  params.grant_type = 'authorization_code';
  const authorization = () => {
    const token = localStorage.getItem('token')
    if (!token) {
      navigate("/login" + location.search);
    } else {
      authorize(params).then(res => {
        if (res.success) {
          window.location.href = res.data;
        } else {
          message.warning('授权失败，请重新登录授权!');
        }
      });
    }
  }

  useEffect(() => {
    if (!localStorage.getItem('token')) {
      navigate("/login" + location.search);
    }
    if (params.client_id == "00000000-0000-0000-0000-000000000001") {
      authorization();
    }
  })

  return (
    <>
      <Button onClick={authorization}>授权</Button>
    </>
  )
}
