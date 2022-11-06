import { useEffect, useState } from "react";
import { fetchClientList } from '@/api/modules/client'


const Client = () => {
  const [tableData, setTableData] = useState();
  const [params, setParams] = useState({ page: 1, size: 10, query: '' });
  const getData = async (params: any) => {
    const res = await fetchClientList(params);
    setTableData(res.data);
  };
  useEffect(() => {
    getData(params);
  }, []);
  return (
    <div>
      {JSON.stringify(tableData)}
    </div>);
}

export default Client;
