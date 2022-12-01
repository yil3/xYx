/**
* @Author xYx
* @Date 2022-11-28 09:40:18
*/

import { Card, List } from "antd";
import VirtualList from "rc-virtual-list";
import { useEffect, useState } from "react";
import { getRolePage } from '@/api/modules/role';

interface RoleItem {
  id: String;
}

export default function Role() {
  const [data, setData] = useState([]);
  const [params, setParams] = useState({ page: 1, size: 10 });
  const appendData = async () => {
    const res = await getRolePage(params);
    if (res.success) {
      setData(data.concat(res.data));
      setParams({ ...params, page: params.page + 1 });
    }

  };
  const onScroll = (e: any) => {
    console.log(e.currentTarget.scrollTop)
    console.log(e.currentTarget.scrollHeight)
    console.log(e.currentTarget.clientHeight)
    if (e.currentTarget.scrollHeight - e.currentTarget.scrollTop === e.currentTarget.clientHeight) {
      console.log("滚动到底部了");
      // setParams({ ...params, page: params.page + 1 });
      // appendData();
    }
  };
  useEffect(() => { appendData() }, []);
  return (
    <List>
      <VirtualList
        itemHeight={50}
        itemKey="id"
        height={200}
        data={data}
        onScroll={onScroll}
      >
        {(item: RoleItem) => (
          <Card>{item.id}</Card>
        )}
      </VirtualList>
    </List>
  );
}
