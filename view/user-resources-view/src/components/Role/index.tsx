/**
* @Author xYx
* @Date 2022-11-28 09:40:18
*/

import { Button, Input, Row, Space, Table } from "antd";
import { useEffect, useState } from "react";
import { getRoleTree } from '@/api/modules/role';
import { PlusOutlined } from "@ant-design/icons";
import { ColumnsType } from "antd/es/table";

interface RoleItem {
  id: String;
  name: String;
  children?: RoleItem[];
}

export default function Role() {
  const [data, setData] = useState([]);
  const [params, setParams] = useState({ query: '' });
  const getData = async () => {
    const res = await getRoleTree(params);
    if (res.success) {
      setData(res.data);
    }
  };

  const columns: ColumnsType<RoleItem> = [
    { title: '角色名称', dataIndex: 'name' },
    { title: '描述', dataIndex: 'description', render: (text: string) => text || '-' },
    { title: '标识', dataIndex: 'code' },
    { title: '创建时间', dataIndex: 'createdAt' },
    {
      title: '操作', width: '20%', render: (_, record: RoleItem) => {
        const addButton = record.children && <Button type="link">添加子角色</Button>;
        return (
          <Space>
            {addButton}
            <Button type="link" size="small">编辑用户</Button>
            <Button type="link" size="small">编辑权限</Button>
            <Button type="link">修改角色</Button>
            <Button type="link">删除</Button>
          </Space>
        )
      }
    }]
  const tableTitle = () => {
    return (
      <Row>
        <Space>
          <Input placeholder="输入名称搜索" />
        </Space>
        <Button style={{ marginLeft: "auto" }} icon={<PlusOutlined />} type="primary">新增</Button>
      </Row>
    )
  }
  useEffect(() => { getData() }, [params]);
  return (
    <>
      <Table rowKey='id' dataSource={data} columns={columns} title={tableTitle} pagination={false} />
    </>
  );
}
