import { Form, Input, message } from "antd";
import { saveClient } from "@/api/modules/client";
import { FormInstance } from "antd/es/form/Form";

interface ClientFormProps {
  form: FormInstance;
  onSaved: () => void;
}

export default function ClientForm(props: ClientFormProps) {
  const save = () => {
    props.form.validateFields().then(async (values: any) => {
      const id = props.form.getFieldValue("id");
      const res = await saveClient({ id, ...values });
      if (res.success) {
        message.success('保存成功');
        props.form.resetFields();
        props.onSaved();
      } else {
        message.error('保存失败');
      }
    });
  }
  return (
    <Form labelCol={{ span: 4 }} name="client" form={props.form} onFinish={save}>
      <Form.Item label="名称" name="name" rules={[{ required: true, message: '请输入客户端名称' }]}>
        <Input />
      </Form.Item>
      <Form.Item label="范围" name="scope" rules={[{ required: true, message: '请输入作用范围' }]}>
        <Input />
      </Form.Item>
      <Form.Item label="回调地址" name="redirectUri" rules={[{ required: true, message: '请输入正确的url', type: 'url' }]}>
        <Input />
      </Form.Item>
      <Form.Item label="密钥" name="secret" rules={[{ required: true, message: '请输入密钥' }]}>
        <Input />
      </Form.Item>
    </Form>
  )
}

