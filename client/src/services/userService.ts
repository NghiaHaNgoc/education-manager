import axios from "axios";
import { Account, User } from "../Model/userModel";
const API_URL = import.meta.env.VITE_API_URL

export const accessToken = JSON.parse(localStorage.getItem('user') as string)?.token
const config = {
    headers : {
        "Content-Type" : "application/json",
        "Authorization" : `Bearer ${accessToken}`
    }
}

export const loginService = async (data : Account) => {
    const res = await axios.post(`${API_URL}/login`, data)
    return res.data
}

export const getObjectsService = async (typeList : string) => {
    const res = await axios.get(`${API_URL}/admin/${typeList}-list`, config)
    return res.data
}

export const createUserService = async (data : User) => {
    const res = await axios.post(`${API_URL}/admin/create-user`, data , config)
    return res.data
}