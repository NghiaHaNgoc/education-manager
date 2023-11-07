import axios from "axios"
import { config } from "."
const API_URL = import.meta.env.VITE_API_URL

export const getClassDetailService = async (codeClass : string) => {
    const res = await axios.get(`${API_URL}/admin/class-detail/${codeClass}` , config())
    return res.data
}

