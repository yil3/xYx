import { useEffect, useState } from "react";
import { fetchClientPage, deleteClient } from '@/api/modules/client'
import { Button, Input, message, Modal, Row, Space, Table } from "antd";
import { PlusOutlined } from "@ant-design/icons";
import ClientForm from './ClientForm';
import useForm from "antd/es/form/hooks/useForm";
import { ColumnsType } from "antd/es/table";

interface TableParams {
  page: number;
  size: number;
  query?: string;
}
/**
* @Author xYx
* @Date 2022-11-13 20:47:06
*/
export default function Client() {
  const [data, setData] = useState<any[]>();
  const [visible, setVisible] = useState(false);
  const [modalTitle, setModalTitle] = useState('');
  const [form] = useForm();
  const [params, setParams] = useState<TableParams>({ page: 1, size: 10 });
  const [total, setTotal] = useState(0);
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
          <a onClick={() => showModal(record)}>编辑</a>
          <a onClick={() => deleteData(record.id)}>删除</a>
        </Space>
      )
    }
  ];
  const getData = async (params: any) => {
    const res = await fetchClientPage(params);
    setData(res.data?.list);
    setTotal(res.data?.total);
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
  const title = () => (
    <Row>
      <Space>
        <Input placeholder="请输入客户端名称搜索" onBlur={e => setParams({ ...params, query: e.currentTarget.value })}/>
      </Space>
      <Button icon={<PlusOutlined />} type="primary" style={{ marginLeft: "auto" }} onClick={showModal}>新增</Button>
    </Row>
  );
  const showModal = (record: any = {}) => {
    setModalTitle(record.id ? '编辑客户端' : '新增客户端');
    form.setFieldsValue(record);
    setVisible(true)
  };
  const handleCancel = () => setVisible(false);
  const handleOk = () => form.submit();
  const saved = () => {
    getData(params);
    setVisible(false);
  };
  const pageChange = (page: number, size: number) => setParams({ ...params, page, size });

  useEffect(() => { getData(params); }, [params]);
  return (
    <>
      <Table rowKey={record => record.id} dataSource={data} columns={columns} pagination={{ onChange: pageChange, total }} title={title} />
      <Modal open={visible} onCancel={handleCancel} title={modalTitle} onOk={handleOk}>
        <ClientForm form={form} onSaved={saved} />
      </Modal>
    </>
  );
}

