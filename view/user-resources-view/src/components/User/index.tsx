import { fetchUserPage } from "@/api/modules/user";
import { ColumnsType } from "antd/es/table";
import { useEffect, useState } from "react";
import { Row, Space, Table, Input, message } from "antd";
/**
* @Author xYx
* @Date 2022-11-26 17:08:39
*/
interface PageParams {
  page: number;
  size: number;
  query?: string;
}
export default function User() {
  const [params, setParams] = useState<PageParams>({ page: 1, size: 10 });
  const [tableData, setTableData] = useState<[any]>();
  const [total, setTotal] = useState(0);

  const columns: ColumnsType<any> = [
    { title: '昵称', dataIndex: 'nickname' },
    { title: 'email', dataIndex: 'email', render: (text: string) => text || '-' },
    { title: '手机号', dataIndex: 'mobile', render: (text: string) => text || '-' },
    { title: '来源', dataIndex: 'origin' },
    { title: '创建时间', dataIndex: 'createdAt' },
    {
      title: '操作', width: '10%',
      render: (_: any, _record: any) => (
        <Space size="middle">
          <a>编辑</a>
          <a>删除</a>
        </Space>
      )
    }
  ];

  const search = (value: string) => {
    setParams({ ...params, query: value });
  }

  const title = () => {
    return (
      <Row>
        <Space>
          <Input placeholder="请输入昵称搜索" onBlur={e => search(e.currentTarget.value)} />
        </Space>
      </Row>
    )
  }
  const getUserPage = async (params: any) => {
    const res = await fetchUserPage(params);
    if (res.success) {
      setTableData(res.data.list);
      setTotal(res.data.total);
    } else {
      message.error(res.message);
    }
  }
  const pageChange = (page: number, size: number) => setParams({ ...params, page, size });

  useEffect(() => { getUserPage(params) }, [params]);

  return (
    <Table rowKey={record => record.id} dataSource={tableData} columns={columns} pagination={{ onChange: pageChange, total }} title={title} />
  );
}
