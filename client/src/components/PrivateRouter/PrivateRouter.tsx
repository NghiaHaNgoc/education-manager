import { Navigate } from "react-router-dom";
import { ReactNode } from 'react';

export default function PrivateRouter({children} : {children : ReactNode}) {

    const dataLogin = '';
  
    if (dataLogin) {
        return children;
    }
    return <Navigate to={"/login"}/>
}