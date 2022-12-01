/**
* @Author xYx
* @Date 2022-11-28 09:40:18
*/

import { Card } from "antd";
import VirtualList from "rc-virtual-list";
import { useEffect, useState } from "react";
import { getRolePage } from '@/api/modules/role';
import "./index.less";

interface RoleItem {
  id: String;
  name: String;
}

export default function Role() {
  const [data, setData] = useState([]);
  const [params, setParams] = useState({ page: 1, size: 10 });
  const [loadover, setLoadover] = useState(false);
  const appendData = async () => {
    const res = await getRolePage(params);
    if (res.success && !data.find((item: RoleItem) => item.id === res.data[0].id)) {
      setData(data.concat(res.data));
    }
    if (res.data.length < params.size) {
      setLoadover(true);
    }

  };
  const onScroll = (e: any) => {
    if (e.currentTarget.scrollHeight - e.currentTarget.scrollTop === e.currentTarget.clientHeight && !loadover) {
      setParams({ ...params, page: params.page + 1 });
    }
  };
  useEffect(() => { appendData() }, [params]);
  return (
    <VirtualList
      height={600}
      itemKey="id"
      data={data}
      onScroll={onScroll}
    >
      {(item: RoleItem) => (
        <Card style={{width: "50px"}}>{item.name}</Card>
      )}
    </VirtualList>
  );
}
