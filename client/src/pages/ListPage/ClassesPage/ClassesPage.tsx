import { useState , useEffect } from "react";
import CascaderComponent from "../../../components/CascaderComponent/CascaderComponent";
import { addStudentsToClassService, createClassService, getClassDetailService, removeStudentsToClassService } from "../../../services/classService";
import { Button, Form, Input, List,Modal,Space,Tabs, Typography } from "antd";
import './Classes.scss'
import TransferComponent from "../../../components/TransferComponent/TransferComponent";
import { toast } from "react-toastify";
import { toastMSGObject } from "../../../utils/utils";
import { getStudentNotClassUser } from "../../../services/userService";
import { DeleteOutlined } from "@ant-design/icons";

interface propsClasses {
  listClasses : any[]
  isUpdateClass : boolean
  setIsUpdateClass ?: any
}

export default function ClassesPage(props : propsClasses) {
  const {
    listClasses,
    isUpdateClass,
    setIsUpdateClass
  } = props;
  const [students , setStudents] = useState([]);
  const [lecturers , setLecturers] = useState([]);
  const [codeClass , setCodeClass] = useState('');
  const [keyTab , setKeyTab] = useState('student');
  const [isOpenModal , setIsOpenModal] = useState(false);
  const [isAddClass , setIsAddClass] = useState(false);
  const [formClass] = Form.useForm();
  const [studentsNotClass , setStudentsNotClass] = useState([]);
  const [studentsSelectedInClass , setStudentsSelectedInClass] = useState([]);
  console.log(studentsSelectedInClass)

  useEffect(() => {
    if(codeClass){
      getClassDetailService(codeClass)
        .then(res => {
          setStudents(
            res['student_in_class']?.map((s : any) => s.student)
          )
          setLecturers(
            res['lecturer_in_class']?.map((s : any) => s.lecturer)
          )
        })
    }else{
      setStudents([]);
      setLecturers([]);
    }
  },[codeClass , isUpdateClass])

  const handleCloseModal = () => {
    setIsOpenModal(false);
    if(isAddClass){
      setIsAddClass(!isAddClass);
      formClass.resetFields();
    }
  }

  // create a new class

  const handleOpenFormAddClass = () => {
    setIsOpenModal(true);
    setIsAddClass(true);
  }

  const handleCreateClass = (valuesInput : any) => {
    createClassService(valuesInput)
      .then(res => {
        setIsUpdateClass(!isUpdateClass);
        handleCloseModal();
      })
      .catch(() => {
        toast.error('This code_class has already exist' , toastMSGObject());
      })
  }

  // add student to class

  const handleGetStudentsNotClass = () => {
    setIsOpenModal(true);
    getStudentNotClassUser()
      .then(res => {
        setStudentsNotClass(res['student_list'])
      })
  }

  const handleAddObjsToClass = () => {
    if(studentsSelectedInClass.length !== 0){
      addStudentsToClassService({
        class : codeClass,
        students : studentsSelectedInClass
      })
        .then(() => {
          toast.success(`There are ${studentsSelectedInClass.length} ${keyTab}s who join in this class`)
          setIsUpdateClass(!isUpdateClass);
          handleCloseModal();
        })
    }else{
      toast(`Please select some ${keyTab}s from available list ${keyTab}s`, toastMSGObject({theme : "dark"}))
    }
  }

  // remove obj out class

  const handleRemoveObjOutClass = (idObj : string) => {
    removeStudentsToClassService({
      class : codeClass,
      students : [idObj]
    })
      .then((res) => {
        toast.success(res.message, toastMSGObject());
        setIsUpdateClass(!isUpdateClass);
      })
  }

  return (
    <div>
      <Tabs
        onChange={(key : string) => setKeyTab(key)}
        tabBarExtraContent = {
          <Space>
            <CascaderComponent 
              options={listClasses?.map(classObj => ({
                value : classObj['class_code'],
                label : classObj['class_code']
              }))}
              placeholder="Chọn hoặc nhập mã lớp học"
              setCodeClass={setCodeClass}
            />
            <Button type="primary" onClick={handleOpenFormAddClass}>Add new class</Button>
          </Space>
        }
        defaultActiveKey={keyTab}
        type="card"
        size="large"
        items={[
          {
            label: `Students`,
            key: `student`,
            children: (
              <List
                header={
                  <div className="header-list">
                    <div className="header-list__title">
                      <span>Student_id</span>
                      <span>Full_name</span>
                    </div>
                    <div className="header-list__total">Total : {students.length} students</div>
                  </div>
                }
                footer={
                  <Button disabled={!codeClass} type="primary" onClick={handleGetStudentsNotClass}>Add students to class</Button>
                }
                bordered
                dataSource={students}
                renderItem={(item) => (
                  <List.Item className="list-item">
                    <div className="list-item__info">
                      <Typography.Text mark style={{marginRight:"28px"}}>{item['student_id']}</Typography.Text> 
                      <span>{item['full_name']}</span>
                    </div>
                    <div onClick={() => handleRemoveObjOutClass(item['student_id'])}>
                      <DeleteOutlined
                        style={{ color: 'red', fontSize: '30px', cursor: 'pointer' }}
                      />
                    </div>
                  </List.Item>
                )}
              />
            ),
          },
          {
            label: `Lecturers`,
            key: `lecturer`,
            children: (
              <List
                header={
                  <div className="header-list">
                    <div className="header-list__title">
                      <span>Lecturer_id</span>
                      <span>Full_name</span>
                    </div>
                    <div className="header-list__total">Total : {lecturers.length} lecturers</div>
                  </div>
                }
                footer={
                  <Button onClick={handleGetStudentsNotClass}>Add Lectures to class</Button>
                }
                bordered
                dataSource={lecturers}
                renderItem={(item) => (
                  <List.Item className="list-item">
                    <div className="list-item__info">
                      <Typography.Text mark style={{marginRight:"28px"}}>{item['student_id']}</Typography.Text> 
                      <span>{item['full_name']}</span>
                    </div>
                    <div>
                      <DeleteOutlined
                        style={{ color: 'red', fontSize: '30px', cursor: 'pointer' }}
                      />
                    </div>
                  </List.Item>
                )}
              />
            ),
          },
        ]}
      />

      <Modal
        width={isAddClass ? 400 : 700} 
        title={isAddClass ? "Add new class" : `Add student to class`} 
        open={isOpenModal} 
        footer={null} 
        onCancel={handleCloseModal}
      >
        {isAddClass ? (
          <Form
            name="Add new class" 
            labelAlign="left" 
            autoComplete="off"
            form={formClass}
            colon={false} // mất dấu : ở label
            labelCol={{ span: 6 }}
            wrapperCol={{ span: 18 }}
            onFinish={handleCreateClass}
          >
            <Form.Item
                label="Class code"
                name="class_code"
            >
                <Input placeholder="class code" />
            </Form.Item>
            <Form.Item
                label="description"
                name="description"
            >
                <Input placeholder="description" />
            </Form.Item>
            <Form.Item
                label=" "
            >
                <Button type="primary" htmlType="submit">Create class</Button>
            </Form.Item>
          </Form>
        ) : (
          <>
            <TransferComponent
              listObjectsAvailable={studentsNotClass}
              keyTab={keyTab}
              isOpenModal={isOpenModal}
              setStudentsSelectedInClass={setStudentsSelectedInClass}
            />
            <Button type="primary" onClick={handleAddObjsToClass}>Confirm Change</Button>          
          </>
        )}
      </Modal>
    </div>
  )
}
