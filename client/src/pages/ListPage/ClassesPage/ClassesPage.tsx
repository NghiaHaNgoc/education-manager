import { useState , useEffect } from "react";
import CascaderComponent from "../../../components/CascaderComponent/CascaderComponent";
import { addObjsToClassService, createClassService, getClassDetailService, removeClassService, removeObjsToClassService} from "../../../services/classService";
import { Button, Form, Input, List,Modal,Popconfirm,Space,Tabs, Typography } from "antd";
import './Classes.scss'
import TransferComponent from "../../../components/TransferComponent/TransferComponent";
import { toast } from "react-toastify";
import { toastMSGObject } from "../../../utils/utils";
import { getObjectNotClass } from "../../../services/userService";
import { DeleteOutlined, PlusOutlined } from "@ant-design/icons";

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
  const [objsNotClass , setObjsNotClass] = useState([]);
  const [objsSelectedInClass , setObjsSelectedInClass] = useState([]);

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
    }else{
      setObjsSelectedInClass([]);
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

  // remove a class

  const handleRemoveClass = () => {
    removeClassService({
      ['class_code'] : codeClass
    })
      .then((res) => {
        toast.success(res.message , toastMSGObject())
        setCodeClass('');
        setIsUpdateClass(!isUpdateClass);
      })
  }

  // add obj to class

  const handleGetObjsNotClass = () => {
    setIsOpenModal(true);
    getObjectNotClass(keyTab)
      .then(res => {
        setObjsNotClass(res[`${keyTab}_list`])
      })
  }

  const handleAddObjsToClass = () => {
    if(objsSelectedInClass.length !== 0){
      addObjsToClassService({
        class : codeClass,
        [`${keyTab}s`] : objsSelectedInClass
      },keyTab)
        .then(() => {
          toast.success(`There are ${objsSelectedInClass.length} ${keyTab}s who join in this class`)
          setIsUpdateClass(!isUpdateClass);
          handleCloseModal();
        })
    }else{
      toast(`Please select some ${keyTab}s from available list ${keyTab}s`, toastMSGObject({theme : "dark"}))
    }
  }

  // remove obj out class

  const handleRemoveObjOutClass = (idObj : string) => {
    removeObjsToClassService({
      class : codeClass,
      [`${keyTab}s`] : [idObj]
    }, keyTab)
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
            <Button 
              style={{
                display:"flex",
                alignItems:"center"
              }}
              type="primary" 
              onClick={handleOpenFormAddClass}
            >
              <PlusOutlined/>
              Add new class
            </Button>
            <Popconfirm
              title="Remove a class"
              description={`Are you sure to remove this class ${codeClass}?`}
              onConfirm={handleRemoveClass}
              okText="Yes"
              cancelText="No"
            >
              <Button 
                type="primary" 
                danger
                disabled={!codeClass}
              >
                remove current class
              </Button>
            </Popconfirm>
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
                  <Button 
                    style={{
                      display:"flex",
                      alignItems:"center"
                    }}
                    disabled={!codeClass} 
                    type="primary" 
                    onClick={handleGetObjsNotClass}
                  >
                    <PlusOutlined/>
                    Add students to class
                  </Button>
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
                  <Button 
                    style={{
                      display:"flex",
                      alignItems:"center"
                    }}
                    disabled={!codeClass} 
                    type="primary" 
                    onClick={handleGetObjsNotClass}
                  >
                    <PlusOutlined style={{marginRight:7}} />
                    Add Lectures to class
                  </Button>
                }
                bordered
                dataSource={lecturers}
                renderItem={(item) => (
                  <List.Item className="list-item">
                    <div className="list-item__info">
                      <Typography.Text mark style={{marginRight:"28px"}}>{item['lecturer_id']}</Typography.Text> 
                      <span>{item['full_name']}</span>
                    </div>
                    <div onClick={() => handleRemoveObjOutClass(item['lecturer_id'])}>
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

      {/** Modal fade cho action add new class and add student or lecturer to class */}
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
              listObjectsAvailable={objsNotClass}
              keyTab={keyTab}
              isOpenModal={isOpenModal}
              setStudentsSelectedInClass={setObjsSelectedInClass}
            />
            <Button disabled={objsSelectedInClass.length===0} type="primary" onClick={handleAddObjsToClass}>Confirm Change</Button>          
          </>
        )}
      </Modal>

    </div>
  )
}
