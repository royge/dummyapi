# Dummy Backend

This a dummy backend API for course management project.

_Base URL:_ [https://dummy-api-ygn33ixetq-as.a.run.app](https://dummy-api-ygn33ixetq-as.a.run.app)

_**WARNING**:_ Your data is not saved in a permanent storage, so it will be
erase when the server terminates. When there is no request coming in for around
5 minutes or more since the last one the server terminates to save costs.

## Supported RESTful APIs

   1. User profile registration
   1. User authentication
   1. Creating and updating courses
   1. Creating and updating course topics
   1. Listing of courses
   1. Listing of course's topics

### 1. User Profile Registration
--------------------------------

   **API Route**: `/profiles`

   **Method**: `POST`

   **Sample Request**

   ```json
   {
      "username": "steve",
      "password": "secret",
      "first_name": "Steve",
      "last_name": "Gates",
      "kind": "admin"
   }
   ```

   **Sample Response**

   _Success_

   ```json
   {
      "data": { "id": 10, "type": "admin" }
   }
   ```

   _Failure_

   ```json
   {
      "error": "Username is no longer available!"
   }
   ```

   **Types of Profile**

   Use one of the following values for the `kind` field.

   - `admin` - If you want to be an organization administration.
   - `teacher` - If you want to be a teacher of the organization.
   - `student` - If you want to study in the organization.

### 2. User Authentication
--------------------------

   **API Route**: `/auth`

   **Method**: `POST`

   **Sample Request**

   ```json
   {
      "username": "steve",
      "password": "secret"
   }
   ```

   **Sample Response**

   _Success_

   ```json
   {
      "data": { "id": 10, "role": "admin", "token": "[JWT]" }
   }
   ```

   _Failure_

   ```json
   {
      "error": "Invalid username or password!"
   }
   ```

### 3. Course Management
------------------------

   ### 3.1 Creating A New Course

   **API Route**: `/courses`

   **Method**: `POST`

   **Sample Request**

   _Header:_

   ```
   Authorization: Bearer [JWT]
   ```

   _Body:_

   ```json
   {
      "title": "Programming Fundamentals",
      "description": "Learn the fundamental concepts of programming."
   }
   ```

   **Sample Response**

   _Success_

   ```json
   {
      "data": { "id": 10, "title": "Title", "description": "Description", "creator_id": 12 }
   }
   ```

   _Failure_

   ```json
   {
      "error": "Title is no longer available!"
   }
   ```

   ### 3.2 Updating Existing Course

   **API Route**: `/courses/{course-id}`

   **Method**: `PUT`

   **Sample Request**

   _Header:_

   ```
   Authorization: Bearer [JWT]
   ```

   _Body:_

   ```json
   {
      "title": "Programming Fundamentals",
      "description": "Learn the fundamental concepts of programming."
   }
   ```

   **Sample Response**

   _Success_

   ```json
   {
      "data": { "id": 10, "title": "Title", "description": "Description", "creator_id": 12 }
   }
   ```

   _Failure_

   ```json
   {
      "error": "Title is no longer available!"
   }
   ```

   ### 3.3 Listing Courses

   **API Route**: `/courses`

   **Method**: `GET`

   **Sample Request**

   _Header:_

   ```
   Authorization: Bearer [JWT]
   ```

   **Sample Response**

   _Success_

   ```json
   {
      "data": [{ "id": 10, "title": "Title", "description": "Description", "creator_id": 12 }]
   }
   ```

   _Failure_

   ```json
   {
      "error": "Not authorized!"
   }
   ```

   ### 3.4 Creating A New Course Topic

   **API Route**: `/topics`

   **Method**: `POST`

   **Sample Request**

   _Header:_

   ```
   Authorization: Bearer [JWT]
   ```

   _Body:_

   ```json
   {
      "title": "Programming Fundamentals",
      "description": "Learn the fundamental concepts of programming.",
      "course_id": 123
   }
   ```

   **Sample Response**

   _Success_

   ```json
   {
      "data": { "id": 10, "title": "Title", "description": "Description", "creator_id": 12, "course_id": 123 }
   }
   ```

   _Failure_

   ```json
   {
      "error": "Title is no longer available!"
   }
   ```

   ### 3.5 Updating Existing Course Topic

   **API Route**: `/topics/{topic-id}`

   **Method**: `PUT`

   **Sample Request**

   _Header:_

   ```
   Authorization: Bearer [JWT]
   ```

   _Body:_

   ```json
   {
      "title": "Programming Fundamentals",
      "description": "Learn the fundamental concepts of programming."
   }
   ```

   **Sample Response**

   _Success_

   ```json
   {
      "data": { "id": 10, "title": "Title", "description": "Description", "creator_id": 12, "course_id": 123 }
   }
   ```

   _Failure_

   ```json
   {
      "error": "Title is no longer available!"
   }
   ```

   ### 3.6 Listing Course Topics

   **API Route**:

       - `/topics`
       - `/topics?course_id={course-id}`

   **Method**: `GET`

   **Sample Request**

   _Header:_

   ```
   Authorization: Bearer [JWT]
   ```

   **Sample Response**

   _Success_

   ```json
   {
      "data": [{ "id": 10, "title": "Title", "description": "Description", "creator_id": 12, "course_id": 123 }]
   }
   ```

   _Failure_

   ```json
   {
      "error": "Not authorized!"
   }
   ```
