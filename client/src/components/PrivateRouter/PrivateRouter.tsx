import { Navigate } from "react-router-dom";
import { ReactNode } from 'react';

export default function PrivateRouter({children} : {children : ReactNode}) {
    const user = JSON.parse(localStorage.getItem('user') as string);
  
    if (user?.token) {
        return children;
    }
    return <Navigate to={"/login"}/>
}