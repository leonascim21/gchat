import axios from "axios";

export interface User {
  id: number;
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
  group_type: number;
}

export interface TempGroup {
  id: number;
  name: string;
  end_date: string;
  temp_chat_key: string;
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
      groups: [],
      friends: [],
      incomingFriends: [],
      outgoingFriends: [],
      tempGroups: [],
    };
  }

  try {
    const [
      userResponse,
      groupsResponse,
      friendsResponse,
      requestsResponse,
      tempGroupsResponse,
    ] = await Promise.all([
      axios.get<User>(
        `https://api.gchat.cloud/user/get-user-info?token=${token}`
      ),
      axios.get<Group[]>(`https://api.gchat.cloud/group/get?token=${token}`),
      axios.get<Friend[]>(`https://api.gchat.cloud/friend/get?token=${token}`),
      axios.get<FriendRequestResponse>(
        `https://api.gchat.cloud/friend/get-requests?token=${token}`
      ),
      axios.get<TempGroup[]>(
        `https://api.gchat.cloud/temp-group/get?token=${token}`
      ),
    ]);

    const user: User | null = userResponse.data;
    const groups: Group[] = groupsResponse.data;
    const tempGroups: TempGroup[] = tempGroupsResponse.data;
    const friends: Friend[] = friendsResponse.data;
    const incomingFriends: FriendRequest[] = requestsResponse.data.incoming;
    const outgoingFriends: FriendRequest[] = requestsResponse.data.outgoing;

    try {
      const memberRequests = groups.map((group) =>
        axios.get<Friend[]>(
          `https://api.gchat.cloud/group/get-users?token=${token}&group_id=${group.id}`
        )
      );

      const memberResponses = await Promise.all(memberRequests);
      groups.forEach((group, i) => {
        group.members = memberResponses[i].data;
      });
    } catch (error) {
      console.error(error);
    }

    return {
      user,
      groups,
      friends,
      incomingFriends,
      outgoingFriends,
      tempGroups,
    };
  } catch (error) {
    console.error("Error fetching data:", error);
    return {
      user: null,
      groups: [],
      friends: [],
      incomingFriends: [],
      outgoingFriends: [],
      tempGroups: [],
    };
  }
}
