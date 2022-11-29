import { Form, FormInstance, Input } from "antd";

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
    props.onSaved()
  }
  return (
    <>
      <Form labelCol={{ span: 4 }} name="client" form={props.form} onFinish={save}>
        <Form.Item label="用户组名称" name="name">
          <Input />
        </Form.Item>
      </Form>
    </>
  )
}
