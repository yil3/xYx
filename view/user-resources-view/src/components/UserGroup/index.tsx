/**
* @Author xYx
* @Date 2022-11-27 20:35:51
*/

import { Button, Input, Modal, Row, Space, Table } from "antd";
import { ColumnsType } from "antd/es/table";
import { fetchUsergroupPage } from "@/api/modules/userGroup";
import { useEffect, useState } from "react";
import { PlusOutlined } from "@ant-design/icons";
import UserGroupForm from "./UserGroupForm";
import { useForm } from "antd/es/form/Form";


export default function UserGroup() {
  const [params, setParams] = useState({ page: 1, size: 10 });
  const [tableData, setTableData] = useState({ total: 0, list: [] });
  const [visible, setVisible] = useState(false);
  const [modalTitle, setModalTitle] = useState("");
  const [form] = useForm();
  const getTableData = async () => {
    const { data } = await fetchUsergroupPage(params);
    setTableData(data);
  };
  const tableTitle = () => {
    return <Row>
      <Space>
        <Input />
      </Space>
      <Button style={{ marginLeft: "auto" }} icon={<PlusOutlined />} type="primary" onClick={showModal}>新增</Button>
    </Row>;
  }
  const showModal = (record: any) => {
    setModalTitle(record.id ? "编辑用户组" : "新增用户组");
    setVisible(true);
    if (record.id) {
    }
  }
  const columns: ColumnsType<any> = [
    { title: '用户组名称', dataIndex: 'name' },
    { title: '用户组描述', dataIndex: 'description', render: (text: string) => text || '-' },
    { title: '创建时间', dataIndex: 'createdAt' },
  ];
  const changePage = (page: number, size: number) => { setParams({ page, size }) };

  const saved = () => {
    console.log('saved');
    setVisible(false);
  }

  const handleOk = () => { form.submit(); setVisible(false); };

  useEffect(() => { getTableData() }, [params]);
  return <>
    <Table rowKey={record => record.id} columns={columns} dataSource={tableData.list}
      pagination={{ onChange: changePage, total: tableData.total }} title={tableTitle}
    />
    <Modal open={visible} onCancel={() => setVisible(false)} title={modalTitle} onOk={handleOk}>
      <UserGroupForm form={form} onSaved={saved} />
    </Modal>
  </>;
}
