import { invoke } from "@tauri-apps/api/core"
import { Friend } from "../types/friend"

export const friendsApi = {
    async fetchFriendsList(): Promise<Friend[]> {
        try {
            console.log("Fetching friends list")
            const account_ids = await invoke<string[]>("fetch_friends_list")
            console.log("Retrieved friends list", account_ids)

            const friends: Friend[] = []
            for (const accountId of account_ids) {
                const name = await invoke<string>("fetch_display_name_by_account_id", { 
                    accountId 
                })
                
                friends.push({
                    id: accountId,
                    name,
                    avatar: "",
                    status: "offline",
                    level: 0
                })
                
                console.log(`Friend ${accountId} | ${name}`)
            }
            
            return friends
        } catch (error) {
            console.error("Error fetching friends list:", error)
            throw error
        }
    },

    async fetchFriendRequests(): Promise<Friend[]> {
        try {
            console.log("Fetching incoming friend requests")
            const account_ids = await invoke<string[]>("fetch_incoming_friends_list")
            console.log("Retrieved incoming friend requests", account_ids)

            const friendRequests: Friend[] = []
            for (const accountId of account_ids) {
                const name = await invoke<string>("fetch_display_name_by_account_id", { 
                    accountId 
                })
                
                friendRequests.push({
                    id: accountId,
                    name,
                    avatar: "",
                    status: "offline",
                    level: 0
                })
                
                console.log(`Friend Request ${accountId} | ${name}`)
            }
            
            return friendRequests
        } catch (error) {
            console.error("Error fetching friend requests:", error)
            throw error
        }
    },

    async acceptFriendRequest(friendAccountId: string): Promise<Friend> {
        try {
            console.log(`Accepting friend request with friend id ${friendAccountId}...`)
            await invoke("accept_friend_request", {
                friendAccountId
            })

            const name = await invoke<string>("fetch_display_name_by_account_id", { 
                accountId: friendAccountId
            })

            const newFriend: Friend = {
                id: friendAccountId,
                name,
                avatar: "",
                status: "offline",
                level: 0
            }

            console.log(`Friend ${friendAccountId} | ${name}`)
            return newFriend
        } catch (error) {
            console.error("Error accepting friend request:", error)
            throw error
        }
    },

    async declineFriendRequest(friendAccountId: string): Promise<void> {
        try {
            console.log(`Declining friend request with friend id ${friendAccountId}...`)
            await invoke("decline_friend_request", {
                friendAccountId
            })
        } catch (error) {
            console.error("Error declining friend request:", error)
            throw error
        }
    }
}