# GChat API

## Overview

A brief, high-level description of what the API does. What problem does it solve? What are its main features? Who is the target audience?

## Base URL

REST API - `https://api.gchat.com/`
Websocket endpoint - `wss://ws.gchat.com/`

## Authentication

How to authenticate with the API. This might include:

- API Keys: How to obtain and use API keys (include example headers or query parameters).
- OAuth 2.0: Details on the OAuth 2.0 flow, including endpoints for authorization, token retrieval, and required scopes.
- Other methods: Basic Authentication, JWT, etc.

Example using an API Key in a header:

```

X-API-Key: YOUR_API_KEY

```

## Endpoints

### User endpoints:

#### `/user/login`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/user/register`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/user/check-token`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/user/get-user-info`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

### Group Endpoints

#### `/group/create`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/group/get-users`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/group/add-users`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/group/remove-user`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/group/edit-picture`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/group/get-messages`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

### Friend Endpoints

#### `/friend/get`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/friend/delete`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/friend/send-request`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/friend/get-requests`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#

#### `/friend/accept-request`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#### `/friend/cancel-request`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```

#### `/friend/deny-request`

- **Description:** A detailed description of what this endpoint does.
- **Method:** `POST`
- **Authentication:** Required? If so, what type?
- **Request Parameters:**

  | Parameter | Type      | Required | Description                      |
  | --------- | --------- | -------- | -------------------------------- |
  | `param1`  | `string`  | Yes      | The first parameter.             |
  | `param2`  | `integer` | No       | The second parameter (optional). |

- **Request Body (for POST/PUT):**

  ```json
  {
    "field1": "value1",
    "field2": 123
  }
  ```

- **Response Codes:**

  | Code | Description                                   |
  | ---- | --------------------------------------------- |
  | 200  | OK - Successful response                      |
  | 201  | Created - Resource successfully created       |
  | 400  | Bad Request - Invalid request parameters      |
  | 401  | Unauthorized - Authentication required        |
  | 403  | Forbidden - Not authorized to access resource |
  | 404  | Not Found - Resource not found                |
  | 500  | Internal Server Error - Something went wrong  |

- **Example Response (Success):**

  ```json
  {
    "id": 123,
    "field1": "value1",
    "field2": 123
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Invalid parameter value."
  }
  ```
