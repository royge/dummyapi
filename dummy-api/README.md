# Dummy Backend

This a dummy backend API for course management project.

## Supported RESTful APIs

   1. User profile registration
   1. User authentication
   1. _[TODO]_ Creating and updating courses
   1. _[TODO]_ Creating and updating course topics

### 1. User Profile Registration

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
      "data": { "id": 10 }
   }
   ```

   _Failure_

   ```json
   {
      "error": "Invalid username or password!"
   }
   ```
