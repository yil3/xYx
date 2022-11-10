import { Form, Input, message, Modal } from "antd";
import { saveClient } from "@/api/modules/client";

const ClientForm = (props: any) => {
  const [form] = Form.useForm();
  const save = () => {
    form.validateFields().then(async (values) => {
      const res = await saveClient(values);
      if (res.success) {
        message.success('保存成功');
        props.handleCancel();
        props.getData(props.params);
        form.resetFields();
      } else {
        message.error('保存失败');
      }
    });
  }
  return (
    <Modal title="客户端表单" open={props.isShow} onCancel={() => props.handleCancel()} onOk={save}>
      <Form labelCol={{ span: 4 }} name="client" form={form}>
        <Form.Item label="客户端名称" name="name">
          <Input />
        </Form.Item>
        <Form.Item label="范围" name="scope">
          <Input />
        </Form.Item>
        <Form.Item label="回调地址" name="redirect_uri">
          <Input />
        </Form.Item>
        <Form.Item label="密钥" name="secret">
          <Input />
        </Form.Item>
      </Form>
    </Modal>
  )
}

export default ClientForm;
