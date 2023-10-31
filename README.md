# Student manager

## [Server link](https://education-manager.fly.dev/fer201m/api)
 - Ensure header set `"Content-Type" : "application/json"`

 - NOTE:
 - `gender` field only accept `Male` and `Female`
 - `birth` field follow format `YYYY-MM-DD` or `YYYY/MM/DD`
 - `role` field only accept `Student`, `Lecturer` and `Admin`

## General
 - This end point for all students, lecturers and admins
### End point `login`
 - Method: `POST`
 - Body:
 ```json
 {
 "username": "your username",
 "password": "your password"
 }
```

### End point `profile`
 - Method: `GET`
 - Make sure included Bearer token in header

### End point `update-profile`
 - Method: `POST`
 - body: field can update is `full_name`, `birth`, `gender`, `address`, `email`, `phone`, `password`
 Example: 
 ```json
 {
 "full_name": "Nghia"
 }
 ```

## Admin
 - API for role admin

### End point `admin/students-list`
 - Method: `GET`
 - Default optional params: `page_number=1`, `students_per_page=24`

### End point `admin/lecturers-list`
 - Method: `GET`
 - Default optional params: `page_number=1`, `lecturers_per_page=24`

### End point `admin/create-user`
 - Method: `POST`
 - Body:  
 ```json
{
    "role": "Student",
    "full_name": "name",
    "birth": "01-01-2003",
    "gender": "Male"
}
 ```
- `role`, `full_name`, `gender`,`birth` are required.
- `address`, `email`, `phone` are optional.
