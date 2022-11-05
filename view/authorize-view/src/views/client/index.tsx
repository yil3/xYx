import { useEffect, useState } from "react";
import { fetchClientList } from '@/api/modules/client'


const Client = () => {
  const [tableData, setTableData] = useState({});
  const getData = async () => {
    const res: any = await fetchClientList({page: 1, size:1});
    setTableData(res.data);
  };
  useEffect(() => {
    getData();
  }, []);
  return (
    <div>
    </div>);
}

export default Client;
