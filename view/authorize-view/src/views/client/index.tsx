import { useEffect, useState } from "react";
import { fetchClientList } from '@/api/modules/client'


const Client = () => {
  const [tableData, setTableData] = useState({});
  const getData = async () => {
    const res: any = await fetchClientList({});
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
