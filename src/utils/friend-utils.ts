import { Friend } from "../types/friend"

export const getStatusColor = (status: string): string => {
    switch (status) {
        case "online":
            return "bg-green-500"
        case "active":
            return "bg-blue-500"
        case "away":
            return "bg-yellow-500"
        default:
            return "bg-gray-500"
    }
}

export const getStatusText = (friend: Friend): string => {
    switch (friend.status) {
        case "online":
            return "Online"
        case "active":
            return "Active on Reality"
        case "away":
            return "Away"
        case "offline":
            return "Offline"
        default:
            return "Unknown"
    }
}

export const filterFriends = (friends: Friend[], searchQuery: string): Friend[] => {
    if (!searchQuery.trim()) return friends
    return friends.filter((friend) => 
        friend.name.toLowerCase().includes(searchQuery.toLowerCase())
    )
}