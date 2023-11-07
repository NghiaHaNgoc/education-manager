import LoadingComponent from "../../components/LoadingComponent/LoadingComponent";
import { useState , useEffect , useMemo} from 'react'
import { createUserService, getObjectsService} from "../../services/userService";
import {toast} from 'react-toastify'
import { toastMSGObject } from "../../utils/utils";
import { useNavigate, useParams } from "react-router-dom";
import TableComponent from "../../components/TableComponent/TableComponent";
import { Modal , Form} from "antd";
import FormComponent from "../../components/FormComponent/FormComponent";
import { Role, Student } from "../../Model/userModel";
import NotFoundPage from "../NotFoundPage/NotFoundPage";
import ClassesPage from "./ClassesPage/ClassesPage";

export default function ListPage() {

  const [isLoading , setIsLoading] = useState<boolean>(true);
  const [listObjects , setListObjects] = useState<any[]>([]);
  const navigate = useNavigate();
  const { type : typeList } = useParams();
  const [isOpenModal , setIsOpenModal] = useState(false);
  const [form] = Form.useForm();
  const [isFormEdit , setIsFormEdit] = useState(false);
  
  const object = useMemo(() => {
    return typeList === 'students' ? 'student' : typeList === 'classes' ? 'class' : 'lecturer'
  },[typeList])

  console.log(JSON.parse(localStorage.getItem('user') as string)?.token)

  // get list objects : students , classes , lecturers
  useEffect(() => {
    getObjectsService(typeList || 'students')
      .then(res => {
        setListObjects(res[`${object}_list`]);
        setIsLoading(false)
      })
      .catch(resFail => {
        toast.error(resFail.response.data.message , toastMSGObject())
        localStorage.removeItem('user');
        navigate('/login')
      })
  },[])

  const handleCancelModal = () => {
    setIsOpenModal(false);
    form.resetFields();
  }

  const handleCreateStudent = (valuesInput : Student) => {
    createUserService({...valuesInput, role : Role[object]})
      .then(() => {
        toast.success('Add new student successfully!' , toastMSGObject())
        handleCancelModal();        
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
                <ClassesPage listClasses={listObjects}/>
              ) : (
                <div>
                  <div>
                    <button onClick={() => setIsOpenModal(true)}>
                      {`Add new ${object}`}
                    </button>
                  </div>
                  <TableComponent
                    typeList={typeList?.toUpperCase()}
                    listData={listObjects}
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
        width={400} 
        title={isFormEdit ? `Edit Information ${object}` : `Add new ${object}`} 
        open={isOpenModal} 
        footer={null} 
        onCancel={handleCancelModal}
      >
        <FormComponent 
          form={form} 
          isFormEdit={isFormEdit} 
          typeList={typeList?.toUpperCase()}
          handleCreate={handleCreateStudent}
        />
      </Modal> 
    </>
  )
}
