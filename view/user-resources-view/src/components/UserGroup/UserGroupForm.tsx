import { Form, FormInstance, Input } from "antd";
import { saveUserGroup } from "@/api/modules/userGroup";

/**
 * @Author xYx
 * @Date 2022-11-29 11:21:55
 */
interface UserGroupFormProps {
  form: FormInstance;
  onSaved: () => void;
}
export default function UserGroupForm(props: UserGroupFormProps) {
  const save = () => {
    props.form.validateFields().then((values) => {
      const id = props.form.getFieldValue("id");
      saveUserGroup({ id, ...values }).then(() => {
        props.onSaved();
      });
    });
  };
  return (
    <>
      <Form
        labelCol={{ span: 4 }}
        name="client"
        form={props.form}
        onFinish={save}
      >
        <Form.Item label="用户组名称" name="name">
          <Input />
        </Form.Item>
        <Form.Item label="用户组描述" name="description">
          <Input.TextArea autoSize={{ minRows: 3 }} />
        </Form.Item>
      </Form>
    </>
  );
}
