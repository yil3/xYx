import { Form, Input, Modal } from "antd";
import { useForm } from "antd/es/form/Form";
import { useEffect, useState } from "react";

/**
* @Author xYx
* @Date 2023-01-11 14:59:44
*/
export interface RoleFormProps {
  visible: boolean;
  onCancel: () => void;
  onSaved: () => void;
  row?: any;
}
export default function RoleForm(props: RoleFormProps) {
  const [title, setTitle] = useState('新增角色');
  const [form] = useForm();
  const save = () => {
    form.validateFields().then(values => {
      values.id = props.row?.id && !props.row?.isAddSubRole ? props.row.id : null;
      values.parentId = props.row?.isAddSubRole ? props.row.id : props.row.parentId == '0' ? '0' : props.row?.parentId || '0';
      // console.log(values)
      form.resetFields();
      props.onSaved();
    });
  }
  const cancel = () => {
    form.resetFields();
    props.onCancel();
  }
  useEffect(() => {
    if (props.row?.isAddSubRole) {
      setTitle('添加子角色');
    } else if (props.row?.id) {
      setTitle('修改角色');
      form.setFieldsValue(props.row);
    } else {
      setTitle('新增角色');
    }
  }, [props.visible, props.row]);
  return (
    <>
      <Modal open={props.visible} title={title} onCancel={cancel} onOk={() => form.submit()}>
        <Form form={form} onFinish={save} name="roleForm" labelCol={{ span: 4 }}>
          <Form.Item name="name" label="角色名称" rules={[{ required: true, message: '请输入角色名称' }]}>
            <Input />
          </Form.Item>
          <Form.Item name="code" label="角色标识" rules={[{ required: true, message: '请输入角色标识' }]} >
            <Input />
          </Form.Item>
          <Form.Item name="description" label="角色描述" >
            <Input.TextArea rows={3} />
          </Form.Item>
        </Form>
      </Modal>
    </>
  )
}
