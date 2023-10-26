# Student manager

## [Server link](https://education-manager.fly.dev/fer201m/api)
 - Ensure header set `"Content-Type" : "application/json"`

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

## Amin
 - API for role admin

### End point `admin/students-list`
 - Method: `GET`
 - Default optional params: `page_number=1`, `students_per_page=24`

### End point `admin/lecturers-list`
 - Method: `GET`
 - Default optional params: `page_number=1`, `lecturers_per_page=24`
