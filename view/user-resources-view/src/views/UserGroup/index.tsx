/**
* @Author xYx
* @Date 2022-11-27 20:35:51
*/

import { Input, Row, Space, Table } from "antd";
import { ColumnsType } from "antd/es/table";
import { fetchUsergroupPage } from "@/api/modules/userGroup";
import { useEffect, useState } from "react";

export default function UserGroup() {
  const [params, setParams] = useState({ page: 1, size: 10 });
  const [tableData, setTableData] = useState({ total: 0, list: [] });
  const getTableData = async () => {
    const { data } = await fetchUsergroupPage(params);
    setTableData(data);
  };
  const tableTitle = () => {
    return <Row>
      <Space>
        <Input />
      </Space>
    </Row>;
  }
  const columns: ColumnsType<any> = [
    { title: '用户组名称', dataIndex: 'name' },
    { title: '用户组描述', dataIndex: 'description', render: (text: string) => text || '-' },
    { title: '创建时间', dataIndex: 'createdAt' },
  ];
  const changePage = (page: number, size: number) => { setParams({ page, size }) };

  useEffect(() => { getTableData() }, [params]);
  return <>
    <Table rowKey={record => record.id} columns={columns} dataSource={tableData.list}
      pagination={{ onChange: changePage, total: tableData.total }} title={tableTitle}
    />
  </>;
}
