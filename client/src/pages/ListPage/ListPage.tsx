import LoadingComponent from "../../components/LoadingComponent/LoadingComponent";
import { useState , useEffect , useMemo} from 'react'
import { createUserService, getObjectDetailService, getObjectsService, removeUser} from "../../services/userService";
import {toast} from 'react-toastify'
import { toastMSGObject } from "../../utils/utils";
import { useNavigate, useParams } from "react-router-dom";
import TableComponent from "../../components/TableComponent/TableComponent";
import { Modal , Form, Descriptions} from "antd";
import FormComponent from "../../components/FormComponent/FormComponent";
import { Role, Student } from "../../Model/userModel";
import NotFoundPage from "../NotFoundPage/NotFoundPage";
import ClassesPage from "./ClassesPage/ClassesPage";
import type { DescriptionsProps } from 'antd';
import { PlusOutlined } from "@ant-design/icons";

export default function ListPage() {

  const [isLoading , setIsLoading] = useState<boolean>(true);
  const [listObjects , setListObjects] = useState<any[]>([]);
  const navigate = useNavigate();
  const { type : typeList } = useParams();
  const [isOpenModal , setIsOpenModal] = useState(false);
  const [form] = Form.useForm();
  const [isFormEdit , setIsFormEdit] = useState(false);
  const [rowSelected , setRowSelected] = useState<any>({});
  const [detailObject , setDetailObject] = useState<any>({});
  const [isUpdateList , setIsUpdateList] = useState(true);
  const [totalObj , setTotalObj] = useState(0);

  const object = useMemo(() => {
    return typeList === 'students' ? 'student' : typeList === 'classes' ? 'class' : 'lecturer'
  },[typeList])

  console.log(rowSelected['student_id'])

  const handleCancelModal = () => {
    setIsOpenModal(false);
    form.resetFields();
  }

  // get list objects : students , classes , lecturers
  useEffect(() => {
    getObjectsService(typeList || 'students')
      .then(res => {
        setListObjects(res[`${object}_list`]);
        setTotalObj(res.total)
        setIsLoading(false)
      })
      .catch(resFail => {
        toast.error(resFail.response.data.message , toastMSGObject())
        localStorage.removeItem('user');
        navigate('/login')
      })
  },[isUpdateList])

  //get object detail 
  useEffect(() => {
    if(isFormEdit && isOpenModal){
      getObjectDetailService(object , rowSelected[`${object}_id`] )
        .then(res => {
          setDetailObject(res);
        })
    }
  },[isOpenModal])

  const mapFieldClassOfObj = (fieldObjInClass : any) => {
    if(object === 'student'){
      return fieldObjInClass?.class['class_code'] || 'Not class'
    }else{
      return fieldObjInClass.map((obj : any) => {
        return obj.class['class_code']
      }).join(', ') || 'Not class'
    }

  }

  const mapObj = () : DescriptionsProps['items'] => {
    let items = [];
    let i = 0;
    for (const key in detailObject) {
      if (detailObject.hasOwnProperty(key)) {
        const value = key===`${object}_in_class` ? mapFieldClassOfObj(detailObject[key]) : detailObject[key];
        items.push({
          key : ++i,
          label : key,
          children : value
        })
      }
    }
    return items
  }

  // create new user

  const handleCreateStudent = (valuesInput : Student) => {
    createUserService({...valuesInput, role : Role[object]})
      .then(() => {
        toast.success('Add new student successfully!' , toastMSGObject())
        handleCancelModal();  
        setIsUpdateList(!isUpdateList)      
      })
  }

  // delete user
  const handleDeleteStudent = () => {
    removeUser({
      user_id : rowSelected[`${object}_id`]
    }).then(res => {
      if(res['code_status'] === 200){
        setIsUpdateList(!isUpdateList)
        toast.success(res.message, toastMSGObject());
      }
    })
  }

  return (
    <>
      {typeList === 'students' || typeList === 'lecturers' || typeList === 'classes' ? (
        <>
          { isLoading ? (
            <LoadingComponent/>
          ) : (
            <>
              {typeList === 'classes' ? (
                <ClassesPage 
                  listClasses={listObjects}
                  isUpdateClass={isUpdateList}
                  setIsUpdateClass={setIsUpdateList}
                />
              ) : (
                <div>
                  <div className="header-list" style={{margin:"15px 0"}}>
                    <div className="header-list__title">{`Total ${typeList} : ${totalObj}`}</div>
                    <button onClick={() => setIsOpenModal(true)} style={{color:"#ffff"}}>
                      <PlusOutlined style={{marginRight:7}} />
                      {`Add new ${object}`}
                    </button>
                  </div>
                  <TableComponent
                    typeList={typeList?.toUpperCase()}
                    listData={listObjects}
                    onRow={(record : any) => {
                      return {
                        onClick : (event : any) => {
                            setRowSelected(record)
                        }
                      }
                    }}
                    setIsOpenModal={setIsOpenModal}
                    setIsFormEdit={setIsFormEdit}
                    handleDeleteStudent={handleDeleteStudent}
                  />
                </div>
              )}
            </>
          )}
        </>
      ) : (
        <NotFoundPage/>
      )}
      <Modal 
        width={isFormEdit ? 1000 : 400} 
        title={isFormEdit ? `Information ${object}` : `Add new ${object}`} 
        open={isOpenModal} 
        footer={null} 
        onCancel={handleCancelModal}
      >
        {isFormEdit ? (
          <Descriptions bordered items={detailObject && mapObj()} />
        ) : (
          <FormComponent 
            form={form} 
            isFormEdit={isFormEdit} 
            typeList={typeList?.toUpperCase()}
            handleCreate={handleCreateStudent}
          />        
        )}
      </Modal> 
    </>
  )
}
