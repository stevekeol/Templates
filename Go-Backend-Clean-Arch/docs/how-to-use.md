# how-to-use

## Example API Request and Response (curl)

- signup

  - Request

  ```shell
  curl --location --request POST 'http://localhost:10000/signup' \
  --data-urlencode 'email=test@gmail.com' \
  --data-urlencode 'password=test' \
  --data-urlencode 'name=Test Name'
  ```

  - Response

  ```json
  {
    "accessToken": "access_token",
    "refreshToken": "refresh_token"
  }
  ```

- login

  - Request

  ```shell
  curl --location --request POST 'http://localhost:10000/login' \
  --data-urlencode 'email=test@gmail.com' \
  --data-urlencode 'password=test'
  ```

  - Response

  ```json
  {
    "accessToken": "access_token",
    "refreshToken": "refresh_token"
  }
  ```

- profile

  - Request

  ```shell
  curl --location --request GET 'http://localhost:10000/profile' \
  --header 'Authorization: Bearer access_token'
  # 需要替换此处的access_token为具体的值
  ```

  - Response

  ```json
  {
    "name": "Test Name",
    "email": "test@gmail.com"
  }
  ```

- task create

  - Request

  ```shell
  curl --location --request POST 'http://localhost:10000/task' \
  --header 'Authorization: Bearer access_token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data-urlencode 'title=Test Task'
  ```

  > 此处服务端提示："message":"invalid character 'È' looking for beginning of value"

  ```json
  {
    "message": "Task created successfully"
  }
  ```

- task fetch

  - Request

  ```shell
  curl --location --request GET 'http://localhost:10000/task' \
  --header 'Authorization: Bearer access_token'
  ```

  - Response

  ```json
  [
    {
      "title": "Test Task"
    },
    {
      "title": "Test Another Task"
    }
  ]
  ```

- refresh token

  - Request

  ```shell
  curl --location --request POST 'http://localhost:10000/refresh' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data-urlencode 'refreshToken=refresh_token'
  ```

  - Response

  ```json
  {
    "accessToken": "access_token",
    "refreshToken": "refresh_token"
  }
  ```
