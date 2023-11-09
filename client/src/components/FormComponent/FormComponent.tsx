import { Button, DatePicker, Form, FormInstance, Input, Radio, Select } from "antd";
import { Student } from "../../Model/userModel";

interface propsForm {
    form : FormInstance<any>,
    isFormEdit : boolean,
    typeList ?: string,
    handleCreate ?: (valuesInput : Student) => void
}

export default function FormComponent(props : propsForm) {

    const {
        form ,
        isFormEdit ,
        typeList ,
        handleCreate 
    } = props

    return (
        <Form 
            name={isFormEdit ? `Edit Information ${typeList}` : `Add new ${typeList}`} 
            // style={{ width: 400 }} 
            // layout="vertical"
            labelAlign="left" 
            autoComplete="off"
            form={form}
            colon={false} // mất dấu : ở label
            labelCol={{ span: 6 }}
            wrapperCol={{ span: 18 }}
            onFinish={handleCreate}
        >
            <Form.Item
                label="Full name"
                name="full_name"
            >
                <Input placeholder="full name , ex : Le Kim Tan" />
            </Form.Item>

            <Form.Item
                label="Birthdate"
                name="birth"
            >
                <DatePicker />
            </Form.Item>

            <Form.Item
                label="Gender"
                name="gender"
            >
                <Radio.Group>
                    <Radio value="Male">MALE</Radio>
                    <Radio value="Female">FEMALE</Radio>
                </Radio.Group>
            </Form.Item>
            <Form.Item
                label=" "
            >
                <Button type="primary" htmlType="submit">Add</Button>
            </Form.Item>
        </Form>
    )
}
