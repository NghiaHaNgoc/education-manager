import axios from "axios";
import { Account } from "../Model/userModel";
const API_URL = import.meta.env.VITE_API_URL

export const loginService = async (data : Account) => {
    const res = await axios.post(`${API_URL}/login`, data)
    return res.data
}