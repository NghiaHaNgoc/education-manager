export interface Account {
    username : string,
    password : string
}

export interface User {
    full_name : string,
    birth : string,
    gender : string,
    password ?: string,
    role ?: Role
}

export enum Role {
    student = 'Student',
    lecturer = 'Lecturer',
    admin = 'Admin',
    class = 'Class'
}

export interface Student extends User{
    student_id ?: string,
}

export interface Lecturer extends User{
    lecturer_id ?: string,
}