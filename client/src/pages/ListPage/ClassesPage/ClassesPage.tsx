import { useState , useEffect } from "react";
import CascaderComponent from "../../../components/CascaderComponent/CascaderComponent";
import { getClassDetailService } from "../../../services/classService";

interface propsClasses {
  listClasses : any[]
}

export default function ClassesPage(props : propsClasses) {
  const {listClasses} = props;
  const [students , setStudents] = useState([]);
  const [lecturers , setlecturers] = useState([]);
  const [codeClass , setCodeClass] = useState('');

  console.log(students)

  useEffect(() => {
    if(codeClass){
      getClassDetailService(codeClass)
        .then(res => {
          setStudents(
            res['student_in_class']?.map((s : any) => s.student)
          )
        })
    }
  },[codeClass])

  return (
    <div>
      <CascaderComponent 
        options={listClasses?.map(classObj => ({
          value : classObj['class_code'],
          label : classObj['class_code']
        }))}
        placeholder="Chọn hoặc nhập mã lớp học"
        setCodeClass={setCodeClass}
      />
    </div>
  )
}
