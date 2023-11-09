import axios from "axios";
import { Account, User } from "../Model/userModel";
import { config } from ".";
const API_URL = import.meta.env.VITE_API_URL

export const loginService = async (data : Account) => {
    const res = await axios.post(`${API_URL}/login`, data)
    return res.data
}

export const getObjectsService = async (typeList : string) => {
    const res = await axios.get(`${API_URL}/admin/${typeList}-list`, config())
    return res.data
}

export const getObjectDetailService = async (object : string , object_id : string) => {
    const res = await axios.get(`${API_URL}/admin/${object}-detail/${object_id}` , config())
    return res.data
}

export const createUserService = async (data : User) => {
    const res = await axios.post(`${API_URL}/admin/create-user`, data , config())
    return res.data
}

export const getProfileUser = async () => {
    const res = await axios.get(`${API_URL}/profile` , config())
    return res.data
}

export const updateProfileUser = async (data : any) => {
    const res = await axios.post(`${API_URL}/profile` , data , config())
    return res.data
}

export const removeUser = async (data : any) => {
    const res = await axios.post(`${API_URL}/admin/remove-user` , data , config())
    return res.data
}

export const getStudentNotClassUser = async () => {
    const res = await axios.get(`${API_URL}/admin/student-not-in-any-class` , config())
    return res.data
}

