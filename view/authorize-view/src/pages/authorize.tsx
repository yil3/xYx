import qs from 'qs';
/**
* @Author xYx
* @Date 2022-11-25 17:16:41
*/
export default function Authorize() {
  // let urlParams = qs.parse(location.search, { ignoreQueryPrefix: true });
  let urlParams = qs.parse(location.search);
  
  const token = localStorage.getItem('token')
  if (!token) {
    window.location.href = "http://localhost:3000/login" + location.search;
  }

  return (
    <></>
  )
}
