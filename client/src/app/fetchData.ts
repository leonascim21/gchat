import axios from "axios";

export interface User {
  userID: number;
  username: string;
  profile_picture: string;
  email: string;
}

export interface Friend {
  friend_id: number;
  username: string;
  profile_picture: string;
}

export interface FriendRequest {
  receiver_id: number;
  sender_id: number;
  username: string;
}

export interface FriendRequestResponse {
  incoming: FriendRequest[];
  outgoing: FriendRequest[];
}

export interface Group {
  id: number;
  name: string;
  profile_picture?: string;
  members: Friend[];
}

export interface Message {
  id: number;
  user_id: number;
  username: string;
  content: string;
  timestamp: string;
  profile_picture: string;
}

export async function fetchAll() {
  const token = localStorage.getItem("token");
  if (!token) {
    console.error("No token found");
    return {
      user: null,
      messages: [],
      groups: [],
      friends: [],
      incomingFriends: [],
      outgoingFriends: [],
    };
  }

  try {
    const [
      messagesResponse,
      userResponse,
      groupsResponse,
      friendsResponse,
      requestsResponse,
    ] = await Promise.all([
      axios.get<Message[]>(
        `https://api.gchat.cloud/get-all-messages?token=${token}`
      ),
      axios.get<User>(`https://api.gchat.cloud/get-user-info?token=${token}`),
      axios.get<Group[]>(`https://api.gchat.cloud/group/get?token=${token}`),
      axios.get<Friend[]>(`https://api.gchat.cloud/friend/get?token=${token}`),
      axios.get<FriendRequestResponse>(
        `https://api.gchat.cloud/friend/get-requests?token=${token}`
      ),
    ]);

    const messages: Message[] = messagesResponse.data;
    const user: User | null = userResponse.data;
    let groups: Group[] = groupsResponse.data;
    const friends: Friend[] = friendsResponse.data;
    const incomingFriends: FriendRequest[] = requestsResponse.data.incoming;
    const outgoingFriends: FriendRequest[] = requestsResponse.data.outgoing;

    for (const group of groups) {
      try {
        const membersResponse = await axios.get<Friend[]>(
          `http://api.gchat.cloud/group/get-users?token=${token}&group_id=${group.id}`
        );
        group.members = membersResponse.data;
      } catch (error) {
        console.error(error);
      }
    }

    groups = [
      { id: -1, name: "Test Chat", profile_picture: "", members: [] },
      ...groups,
    ];

    return {
      user,
      messages,
      groups,
      friends,
      incomingFriends,
      outgoingFriends,
    };
  } catch (error) {
    console.error("Error fetching data:", error);
    return {
      user: null,
      messages: [],
      groups: [],
      friends: [],
      incomingFriends: [],
      outgoingFriends: [],
    };
  }
}
