import axios from "axios"
import { config } from "."
const API_URL = import.meta.env.VITE_API_URL

export const getClassDetailService = async (codeClass : string) => {
    const res = await axios.get(`${API_URL}/admin/class-detail/${codeClass}` , config())
    return res.data
}

export const createClassService = async (data : any) => {
    const res = await axios.post(`${API_URL}/admin/create-class`, data , config())
    return res.data
}

export const updateClassService = async (data : any , classCode : string) => {
    const res = await axios.post(`${API_URL}/admin/update-class/${classCode}`, data , config())
    return res.data
}

export const addStudentsToClassService = async (data : any ) => {
    const res = await axios.post(`${API_URL}/admin/add-students-to-class`, data , config())
    return res.data
}

export const addLecturersToClassService = async (data : any ) => {
    const res = await axios.post(`${API_URL}/admin/add-lecturers-to-class`, data , config())
    return res.data
}

export const removeStudentsToClassService = async (data : any ) => {
    const res = await axios.post(`${API_URL}/admin/remove-lecturers-to-class`, data , config())
    return res.data
}

export const removeLecturersToClassService = async (data : any ) => {
    const res = await axios.post(`${API_URL}/admin/remove-lecturers-to-class`, data , config())
    return res.data
}


