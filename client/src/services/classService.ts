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

export const removeClassService = async (data : any) => {
    const res = await axios.post(`${API_URL}/admin/remove-class`, data , config())
    return res.data
}

export const updateClassService = async (data : any , classCode : string) => {
    const res = await axios.post(`${API_URL}/admin/update-class/${classCode}`, data , config())
    return res.data
}

export const addObjsToClassService = async (data : any , obj : string ) => {
    const res = await axios.post(`${API_URL}/admin/add-${obj}s-to-class`, data , config())
    return res.data
}

export const removeObjsToClassService = async (data : any , obj: string ) => {
    const res = await axios.post(`${API_URL}/admin/remove-${obj}s-from-class`, data , config())
    return res.data
}

export const detailClassByObjService = async (classCode : string , obj : string) => {
    const res = await axios.get(`${API_URL}/${obj}/class-detail/${classCode}`, config())
    return res.data
}



