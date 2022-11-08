import { useEffect, useState } from "react";
import { fetchClientList } from '@/api/modules/client'
import { Space, Table, TablePaginationConfig } from "antd";
import Column from "antd/lib/table/Column";

interface TableParams {
  page: number;
  size: number;
  query?: string;
  pagination?: TablePaginationConfig;
}

const Client = () => {
  const [data, setData] = useState<any>();
  const [params, setParams] = useState<TableParams>({
    page: 1,
    size: 10,
    query: '',
    pagination: {
      current: 1,
      pageSize: 10,
    },
  });
  const getData = async (params: any) => {
    const res = await fetchClientList(params);
    setData(res.data?.list);
    setParams({
      ...params,
      pagination: {
        current: params.page,
        pageSize: params.size,
        total: res.data?.total,
      }
    });
  };
  useEffect(() => {
    getData(params);
  }, []);
  return (
    <div>
      <Table dataSource={data} pagination={params.pagination} onChange={pagination => {
        getData({ page: pagination.current, size: pagination.pageSize });
      }}>
        <Column title="客户端名称" dataIndex="name" key="name"></Column>
        <Column title="scope" dataIndex="scope" key="scope"></Column>
        <Column title="redirect_uri" dataIndex="redirectUri" key="redirectUri"></Column>
        <Column title="secret" dataIndex="secret" key="secret"></Column>
        <Column title="拥有者" dataIndex="owner" key="owner"></Column>
        <Column title="创建时间" dataIndex="createdAt" key="createdAt"></Column>
        <Column title="更新时间" dataIndex="updatedAt" key="updatedAt"></Column>
        <Column title="操作" dataIndex="action" key="action" render={
          (_, record) => {
            return (
              <Space size="middle">
                <a onClick={() => { console.log("record", record) }}>编辑</a>
                <a>删除</a>
              </Space>
            )
          }
        }></Column>
      </Table>
    </div>
  );
}

export default Client;
