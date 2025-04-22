# GChat API

1.  [Overview](#overview)
2.  [Base URL](#base-url)
3.  [Authentication](#authentication)
4.  [User Endpoints](#user-endpoints)
    - [Login](#userlogin)
    - [Register](#userregister)
    - [Check Token](#usercheck-token)
    - [Get User Info](#userget-user-info)
5.  [Group Chat Endpoints](#group-endpoints)
    - [Create Group Chat](#groupcreate)
    - [Get Group Chats](#groupget)
    - [Get Group Members](#groupget-users)
    - [Add Members](#groupadd-users)
    - [Remove Member](#groupremove-user)
    - [Edit Group Picture](#groupedit-picture)
    - [Get Group Messages](#groupget-messages)
6.  [Temporary Group Chat Endpoints](#temporary-group-chat-endpoints)
    - [Get Temporary Group Messages](#temp-groupget-messages)
    - [Get Temporary Group Info](#temp-groupget-group-info)
    - [Check if Group has a Password](#temp-grouphas-password)
    - [Create Temprorary Group Chat](#temp-groupcreate)
    #### TODO: write /temp-group/get documentation
    #### ALTER DOCS PASSWORD REQUIRED ON TEMP CHATS
7.  [Friend Endpoints](#friend-endpoints)
    - [Get Friends](#friendget)
    - [Remove Friend](#frienddelete)
    - [Send Friend Request](#friendsend-request)
    - [Get Friend Requests](#friendget-requests)
    - [Accept Friend Request](#friendaccept-request)
    - [Cancel Friend Request](#friendcancel-request)
    - [Deny Friend Request](#frienddeny-request)
    #### TODO: ADD WEBSOKCET DOCS

## Overview

#### TODO

## Base URL

REST API - `https://api.gchat.com/`
Websocket endpoint - `wss://ws.gchat.com/`

## Authentication

Authentication with the GChat API is primarily done using JWT (JSON Web Tokens).

After a user logs in or registers, a JWT is issued. This token must be included in subsequent requests.

## Endpoints

### User endpoints

#### `/user/login`

- **Description:** Authenticates a user and returns a JWT token.
- **Method:** `POST`
- **Authentication:** Not required.

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
username=string&password=string
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Successful login, returns JWT           |
| 401  | Unauthorized - Invalid credentials           |
| 400  | Bad Request - Invalid request parameters     |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "token": "YOUR_JWT_TOKEN"
}
```

- \*\*Example Response (Error):

```json
{
  "error": "Invalid credentials."
}
```

#### `/user/register`

- **Description:** Registers a new user and returns a JWT token.
- **Method:** `POST`
- **Authentication:** Not required.

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
username=string&email=string&password=string&confirmPassword=string&profilePicture=string
```

- **Response Codes:**

| Code | Description                                                      |
| ---- | ---------------------------------------------------------------- |
| 200  | OK - Successful registration, returns JWT                        |
| 400  | Bad Request - Invalid request parameters, passwords do not match |
| 500  | Internal Server Error - Something went wrong                     |

- **Example Response (Success):**

```json
{
  "token": "YOUR_JWT_TOKEN"
}
```

- \*\*Example Response (Error):

```json
{
  "error": "Username or email already exists."
}
```

#### `/user/check-token`

- **Description:** Checks if a JWT token is valid.
- **Method:** `GET`
- **Authentication:** Not required.
- **Request Parameters:**

| Parameter | Type     | Required | Description    |
| --------- | -------- | -------- | -------------- |
| `token`   | `string` | Yes      | The JWT token. |

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Returns validation status               |
| 401  | Unauthorized - Invalid token                 |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "valid": true
}
```

- **Example Response (Error):**

```json
{
  "valid": false
}
```

#### `/user/get-user-info`

- **Description:** Retrieves user information based on a valid JWT token.
- **Method:** `GET`
- **Authentication:** Required (JWT as query parameter).
- **Request Parameters:**

| Parameter | Type     | Required | Description    |
| --------- | -------- | -------- | -------------- |
| `token`   | `string` | Yes      | The JWT token. |

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Returns user information                |
| 401  | Unauthorized - Invalid or missing token      |
| 404  | Not Found - User not found                   |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "id": 123,
  "username": "testuser",
  "email": "test@example.com",
  "profile_picture": "url"
}
```

- **Example Response (Error):**

```json
{
  "error": "Invalid token."
}
```

### Group Endpoints

#### `/group/create`

- **Description:** Creates a new group chat.
- **Method:** `POST`
- **Authentication:** Required (JWT in POST body).

- **Request Body (for POST/PUT):**

```json
{
  "token": "YOUR_JWT_TOKEN",
  "group_name": "string",
  "member_ids": [1, 2, 3]
}
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Successful group creation               |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "Group Created",
  "group_id": 4
}
```

- **Example Response (Error):**

```json
{
  "error": "Internal Server Error"
}
```

#### `/group/get`

- **Description:** Retrieves groups for a given user ID.
- **Method:** `GET`
- **Authentication:** Required (JWT as query parameter).
- **Request Parameters:**

| Parameter | Type     | Required | Description    |
| --------- | -------- | -------- | -------------- |
| `token`   | `string` | Yes      | The JWT token. |

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Returns group information               |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
[
  {
    "name": "group1",
    "profile_picture": "url",
    "id": 1
  }
]
```

- **Example Response (Error):**

```json
{
  "error": "Failed to fetch friendships"
}
```

#### `/group/get-users`

- **Description:** Retrieves users in a specific group.
- **Method:** `GET`
- **Authentication:** Required (JWT as query parameter).
- **Request Parameters:**

| Parameter  | Type     | Required | Description          |
| ---------- | -------- | -------- | -------------------- |
| `token`    | `string` | Yes      | The JWT token.       |
| `group_id` | `string` | Yes      | The ID of the group. |

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Returns users in group                  |
| 400  | Bad Request - Missing group_id               |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
[
  {
    "username": "user1",
    "profile_picture": "url",
    "friend_id": 2
  }
]
```

- **Example Response (Error):**

```json
{
  "error": "User not in group"
}
```

#### `/group/add-users`

- **Description:** Adds users to an existing group.
- **Method:** `POST`
- **Authentication:** Required (JWT in POST body).

- **Request Body (for POST/PUT):**

```json
{
  "token": "YOUR_JWT_TOKEN",
  "group_id": 1,
  "new_member_ids": [4, 5]
}
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Users added successfully                |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "Users Added"
}
```

- **Example Response (Error):**

```json
{
  "error": "Internal Server Error"
}
```

#### `/group/remove-user`

- **Description:** Removes a user from a group.
- **Method:** `POST`
- **Authentication:** Required (JWT in POST body).

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
token=YOUR_JWT_TOKEN&group_id=1&remove_id=4
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - User removed successfully               |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "User Removed"
}
```

- **Example Response (Error):**

```json
{
  "error": "Failed to remove user"
}
```

#### `/group/edit-picture`

- **Description:** Edits the profile picture of a group.
- **Method:** `PUT`
- **Authentication:** Required (JWT in PUT body).

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
token=YOUR_JWT_TOKEN&group_id=1&picture_url=new_url
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Picture updated successfully            |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "Picture Updated"
}
```

- **Example Response (Error):**

```json
{
  "error": "Failed to update picture"
}
```

#### `/group/get-messages`

- **Description:** Retrieves messages from a group.
- **Method:** `GET`
- **Authentication:** Required (JWT in query parameters).
- **Request Parameters:**

| Parameter  | Type     | Required | Description          |
| ---------- | -------- | -------- | -------------------- |
| `token`    | `string` | Yes      | The JWT token.       |
| `group_id` | `string` | Yes      | The ID of the group. |

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Returns messages                        |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
[
  {
    "id": 1,
    "content": "Hello!",
    "user_id": 2,
    "username": "testuser",
    "timestamp": "2024-01-01T00:00:00Z",
    "profile_picture": "url"
  }
]
```

- **Example Response (Error):**

```json
{
  "error": "Failed to fetch messages"
}
```

### Temporary Group Chat Endpoints

#### `/temp-group/get-messages`

- **Description:** Retrieves messages from a temporary group chat.
- **Method:** `GET`
- **Authentication:** Required (temporary chat key as query parameter).
- **Request Parameters:**

  | Parameter  | Type     | Required | Description                                                              |
  | :--------- | :------- | :------- | :----------------------------------------------------------------------- |
  | `temp`     | `string` | Yes      | The temporary chat key for the group.                                    |
  | `password` | `string` | No       | The password for the group, required if the group is password-protected. |

- **Response Codes:**

  | Code | Description                                                    |
  | :--- | :------------------------------------------------------------- |
  | 200  | OK - Returns messages from the group.                          |
  | 401  | Unauthorized - Missing chat key or incorrect password.         |
  | 500  | Internal Server Error - Failed to fetch chat info or messages. |

- **Example Response (Success):**

  ```json
  [
    {
      "id": 1,
      "content": "Hello!",
      "user_id": 2,
      "username": "testuser",
      "timestamp": "2024-01-01T00:00:00Z",
      "profile_picture": "url"
    }
  ]
  ```

- **Example Response (Error - Unauthorized):**

  ```json
  {
    "error": "Unauthorized"
  }
  ```

#### `/temp-group/get-group-info`

- **Description:** Retrieves information about a temporary group chat.
- **Method:** `GET`
- **Authentication:** Required (temporary chat key as query parameter).
- **Request Parameters:**

  | Parameter  | Type     | Required | Description                                                              |
  | :--------- | :------- | :------- | :----------------------------------------------------------------------- |
  | `temp`     | `string` | Yes      | The temporary chat key for the group.                                    |
  | `password` | `string` | No       | The password for the group, required if the group is password-protected. |

- **Response Codes:**

  | Code | Description                                            |
  | :--- | :----------------------------------------------------- |
  | 200  | OK - Returns group information.                        |
  | 401  | Unauthorized - Missing chat key or incorrect password. |
  | 500  | Internal Server Error - Failed to fetch chat info.     |

- **Example Response (Success):**

  ```json
  {
    "chat_key": "unique_chat_key",
    "group_id": 123,
    "end_date": "2024-01-01T00:00:00Z",
    "name": "Temporary Group"
  }
  ```

- **Example Response (Error - Unauthorized):**

  ```json
  {
    "error": "Unauthorized"
  }
  ```

#### `/temp-group/has-password`

- **Description:** Checks if a temporary group chat is password-protected.
- **Method:** `GET`
- **Authentication:** Required (temporary chat key as query parameter).
- **Request Parameters:**

  | Parameter | Type     | Required | Description             |
  | :-------- | :------- | :------- | :---------------------- |
  | `temp`    | `string` | Yes      | The temporary chat key. |

- **Response Codes:**

  | Code | Description                                        |
  | :--- | :------------------------------------------------- |
  | 200  | OK - Returns whether the group has a password.     |
  | 401  | Unauthorized - Missing chat key.                   |
  | 500  | Internal Server Error - Failed to fetch chat info. |

- **Example Response (Has Password):**

  ```json
  {
    "has_password": true
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "error": "Failed to fetch chat info"
  }
  ```

#### `/temp-group/create`

- **Description:** Creates a new temporary group chat.
- **Method:** `POST`
- **Authentication:** Not required.
- **Request Body (for POST/PUT):**

  ```json
  {
    "group_name": "string",
    "end_date": "string",
    "password": "string (optional, leave empty for no password)"
  }
  ```

- **Response Codes:**

  | Code | Description                                                         |
  | :--- | :------------------------------------------------------------------ |
  | 200  | OK - Successful group creation, returns chat key.                   |
  | 400  | Bad Request - Invalid end date format.                              |
  | 500  | Internal Server Error - Something went wrong during group creation. |

- **Example Response (Success):**

  ```json
  {
    "message": "Group Created",
    "chat_key": "temp_chat_key"
  }
  ```

- **Example Response (Error):**

  ```json
  {
    "message": "Internal Server Error"
  }
  ```

#

### Friend Endpoints

#### `/friend/get`

- **Description:** Retrieves a list of friends for a user.
- **Method:** `GET`
- **Authentication:** Required (JWT in query parameters).
- **Request Parameters:**

| Parameter | Type     | Required | Description    |
| --------- | -------- | -------- | -------------- |
| `token`   | `string` | Yes      | The JWT token. |

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Returns list of friends                 |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
[
  {
    "friend_id": 2,
    "username": "friend1"
  }
]
```

- **Example Response (Error):**

```json
{
  "error": "Failed to fetch friendships"
}
```

#### `/friend/delete`

- **Description:** Removes a friend from a user's friend list.
- **Method:** `POST`
- **Authentication:** Required (JWT in POST body).

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
token=YOUR_JWT_TOKEN&user_id=2
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Friend removed successfully             |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "Friend removed"
}
```

- **Example Response (Error):**

```json
{
  "error": "Failed to remove friend"
}
```

#### `/friend/send-request`

- **Description:** Sends a friend request to a user.
- **Method:** `POST`
- **Authentication:** Required (JWT in POST body).

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
token=YOUR_JWT_TOKEN&receiver_username=string
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Friend request sent successfully        |
| 400  | Bad Request - User not found                 |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "Friend request sent successfully",
  "friend_request": {}
}
```

- **Example Response (Error):**

```json
{
  "error": "Internal server error, please try again"
}
```

#### `/friend/get-requests`

- **Description:** Retrieves pending friend requests for a user.
- **Method:** `GET`
- **Authentication:** Required (JWT in query parameters).
- **Request Parameters:**

| Parameter | Type     | Required | Description    |
| --------- | -------- | -------- | -------------- |
| `token`   | `string` | Yes      | The JWT token. |

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Returns pending friend requests         |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "outgoing": [
    {
      "sender_id": 1,
      "receiver_id": 2,
      "username": "user2"
    }
  ],
  "incoming": [
    {
      "sender_id": 3,
      "receiver_id": 1,
      "username": "user3"
    }
  ]
}
```

- **Example Response (Error):**

```json
{
  "error": "Failed to fetch friend requests"
}
```

#### `/friend/accept-request`

- **Description:** Accepts a friend request.
- **Method:** `POST`
- **Authentication:** Required (JWT in POST body).

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
token=YOUR_JWT_TOKEN&user_id=3
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Friend request accepted                 |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "Friend request accepted"
}
```

- **Example Response (Error):**

```json
{
  "error": "Failed to accept friend request"
}
```

#### `/friend/cancel-request`

- **Description:** Cancels a sent friend request.
- **Method:** `POST`
- **Authentication:** Required (JWT in POST body).

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
token=YOUR_JWT_TOKEN&user_id=2
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Friend request canceled                 |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "Friend request canceled"
}
```

- **Example Response (Error):**

```json
{
  "error": "Failed to cancel friend request"
}
```

#### `/friend/deny-request`

- **Description:** Denies an incoming friend request.
- **Method:** `POST`
- **Authentication:** Required (JWT in POST body).

- **Request Body (for POST/PUT - x-www-form-urlencoded):**

```
token=YOUR_JWT_TOKEN&user_id=3
```

- **Response Codes:**

| Code | Description                                  |
| ---- | -------------------------------------------- |
| 200  | OK - Friend request denied                   |
| 401  | Unauthorized - Invalid or missing token      |
| 500  | Internal Server Error - Something went wrong |

- **Example Response (Success):**

```json
{
  "message": "Friend request denied"
}
```

- **Example Response (Error):**

```json
{
  "error": "Failed to deny friend request"
}
```
