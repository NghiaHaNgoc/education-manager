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
    "full_name": "Nghia",
    "email": "nghia@example.com"
 }
 ```

## Admin
 - API for role admin

### End point `admin/students-list`
 - Method: `GET`
 - Default optional params: `page_number=1`, `students_per_page=24`

### End point `admin/student-not-in-any-class`
 - Method: `GET`
 - Default optional params: `page_number=1`, `students_per_page=24`

### End point `admin/lecturers-list`
 - Method: `GET`
 - Default optional params: `page_number=1`, `lecturers_per_page=24`

### End point `admin/classes-list`
 - Method: `GET`
 - Default optional params: `page_number=1`, `classes_per_page=24`


### End point `admin/student-detail/:student_id`
 - Method: `GET`
 - `student_id` in path is required

### End point `admin/lecturer-detail/:lecturer_id`
 - Method: `GET`
 - `lecturer_id` in path is required

### End point `admin/class-detail/:class_code`
 - Method: `GET`
 - `class_code` in path is required

### End point `admin/create-user`
 - Method: `POST`
 - Body:  
 ```json
{
    "role": "Student",
    "full_name": "name",
    "birth": "2003-01-01",
    "gender": "Male"
}
 ```
- `role`, `full_name`, `gender`,`birth` are required.
- `address`, `email`, `phone` are optional.

### End point `admin/create-class`
 - Method: `POST`
 - Body:  
 ```json
{
    "class_code": "10A01",
    "description": "Class 10A01"
}
 ```
- `class_code` is required.
- `description` is optional.

### End point `admin/add-students-to-class`
- Method: `POST`
- Body:
```json
{
    "class": "10A01",
    "students": ["ST00001", "ST00002"]
}
```

### End point `admin/add-lecturers-to-class`
- Method: `POST`
- Body:
```json
{
    "class": "10A01",
    "students": ["LT00001", "LT00002"]
}
```

### End point `admin/update-class/:class_code`
- Method: `POST`
- Body:
```json
{
    "class_code": "new class code",
    "description": "new description"
}
```
- `class_code` in path is required
- `class_code` and `description` in body are optional

### End point `admin/remove-user`
 - Method: `POST`
 - Body:  
 ```json
{
    "user_id": "ST00001"
}
 ```
- `user_id` is required.

### End point `admin/remove-class`
 - Method: `POST`
 - Body:  
 ```json
{
    "class_code": "20A01"
}
 ```
- `class_code` is required.

### End point `admin/remove-students-from-class`
 - Method: `POST`
 - Body:  
 ```json
{
    "class": "20A01",
    "students": ["ST00001", "ST00002"]
}
 ```
- `class` and `students` are required.

### End point `admin/remove-lecturers-from-class`
 - Method: `POST`
 - Body:  
 ```json
{
    "class": "20A01",
    "lecturers": ["LT00001", "LT00002"]
}
 ```
- `class` and `lecturers` are required.

## Lecturer

- End point for lecturer role

### End point `lecturer/class-detail/:class_code`
 - Method: `GET`
 - `class_code` in path is required

## Student

- End point for student role

### End point `student/class-detail/:class_code`
 - Method: `GET`
 - `class_code` in path is required

