import { Button, Input, message, Modal, Row, Space, Table } from "antd";
import { ColumnsType } from "antd/es/table";
import { fetchUsergroupPage } from "@/api/modules/userGroup";
import { useEffect, useState } from "react";
import { PlusOutlined } from "@ant-design/icons";
import UserGroupForm from "./UserGroupForm";
import { useForm } from "antd/es/form/Form";
import { deleteUserGroup } from "@/api/modules/userGroup";

/**
 * @Author xYx
 * @Date 2022-11-27 20:35:51
 */
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
    return (
      <Row>
        <Space>
          <Input placeholder="输入名称搜索" />
        </Space>
        <Button style={{ marginLeft: "auto" }} icon={<PlusOutlined />} type="primary" onClick={showModal}>新增</Button>
      </Row>
    );
  };
  const showModal = (record: any) => {
    form.setFieldsValue(record);
    setModalTitle(record.id ? "编辑用户组" : "新增用户组");
    setVisible(true);
  };
  const columns: ColumnsType<any> = [
    { title: "用户组名称", dataIndex: "name" },
    { title: "创建时间", dataIndex: "createdAt" },
    { title: "用户组描述", dataIndex: "description", render: (text: string) => text || "-", },
    {
      title: "操作",
      width: "10%",
      render: (_text: string, record: any) => (
        <Space>
          <Button type="link" onClick={() => showModal(record)} > 编辑 </Button>
          <Button type="link" onClick={() => deleteRecord(record.id)}> 删除 </Button>
        </Space>
      ),
    },
  ];
  const deleteRecord = (id: String) => {
    Modal.confirm({
      title: "提示",
      content: "确定删除该用户组吗？",
      onOk: async () => {
        const res = await deleteUserGroup(id);
        if (res.success) {
          getTableData();
        } else {
          message.error(res.message);
        }
      },
    });
  };
  const changePage = (page: number, size: number) => setParams({ ...params, page, size });

  const saved = () => {
    setVisible(false);
    getTableData();
  };

  const handleOk = () => {
    form.submit();
    setVisible(false);
  };

  useEffect(() => { getTableData() }, [params]);

  return (
    <>
      <Table
        rowKey={(record) => record.id}
        columns={columns}
        dataSource={tableData.list}
        pagination={{ onChange: changePage, total: tableData.total }}
        title={tableTitle}
      />
      <Modal
        open={visible}
        onCancel={() => setVisible(false)}
        title={modalTitle}
        onOk={handleOk}
      >
        <UserGroupForm form={form} onSaved={saved} />
      </Modal>
    </>
  );
}
