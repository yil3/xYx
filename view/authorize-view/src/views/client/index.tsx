import { useEffect, useState } from "react";
import { fetchClientList, deleteClient } from '@/api/modules/client'
import { Button, Input, message, Modal, Row, Space, Table, TablePaginationConfig } from "antd";
import { PlusOutlined } from "@ant-design/icons";
import ClientForm from './ClientForm';
import { ColumnsType } from "antd/lib/table";

interface TableParams {
  page: number;
  size: number;
  query?: string;
  pagination?: TablePaginationConfig;
}

const Client = () => {
  const [data, setData] = useState();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [params, setParams] = useState<TableParams>({
    page: 1,
    size: 10,
  });
  const columns: ColumnsType<any> = [
    { title: '客户端名称', dataIndex: 'name' },
    { title: '范围', dataIndex: 'scope' },
    { title: '回调地址', dataIndex: 'redirectUri' },
    { title: '拥有者', dataIndex: 'owner' },
    { title: '密钥', dataIndex: 'secret' },
    { title: '创建时间', dataIndex: 'createdAt', width: '10%' },
    { title: '更新时间', dataIndex: 'updatedAt', width: '10%' },
    {
      title: '操作', width: '10%',
      render: (_: any, record: any) => (
        <Space size="middle">
          <a onClick={() => { console.log("record", record) }}>编辑</a>
          <a onClick={() => deleteData(record.id)}>删除</a>
        </Space>
      )
    }
  ];
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
  const deleteData = async (id: string) => {
    Modal.confirm({
      title: '提示',
      content: '确定删除吗？',
      onOk: async () => {
        const res = await deleteClient(id);
        if (res.success) {
          message.success('删除成功');
          getData(params);
        } else {
          message.error('删除失败');
        }
      }
    });
  };
  const pageChange = (pagination: any) => {
    setParams({
      ...params,
      page: pagination.current,
      size: pagination.pageSize,
    });
    getData({
      ...params,
      page: pagination.current,
      size: pagination.pageSize,
    });
  }
  const title = () => (
    <Row>
      <Space>
        <Input placeholder="请输入客户端名称搜索" />
      </Space>
      <Button icon={<PlusOutlined />} type="primary" style={{ marginLeft: "auto" }} onClick={showModal}>新增</Button>
    </Row>
  );
  const showModal = () => {
    setIsModalOpen(true);
  };

  const handleCancel = () => {
    setIsModalOpen(false);
  };
  useEffect(() => {
    getData(params);
  }, []);
  return (
    <>
      <Table rowKey={record => record.id} dataSource={data} columns={columns} pagination={params.pagination} onChange={pageChange} title={title} />
      <ClientForm isShow={isModalOpen} handleCancel={handleCancel} getData={getData} params={params} />
    </>
  );
}

export default Client;
