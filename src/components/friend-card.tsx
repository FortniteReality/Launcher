import { Avatar, AvatarFallback, AvatarImage } from "./ui/avatar"
import { Users, Clock, Gamepad2 } from "lucide-react"
import { Friend } from "../types/friend"
import { getStatusColor, getStatusText } from "../utils/friend-utils"

interface FriendCardProps {
    friend: Friend
}

export default function FriendCard({ friend }: FriendCardProps) {
    return (
        <div className="backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-4 hover:bg-black/40 transition-all duration-300">
            <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                    <div className="relative">
                        <Avatar className="h-12 w-12">
                            <AvatarImage src={friend.avatar || "/placeholder.svg"} />
                            <AvatarFallback className="bg-gray-800 text-gray-300">
                                {friend.name
                                    .split(" ")
                                    .map((n) => n[0])
                                    .join("")}
                            </AvatarFallback>
                        </Avatar>
                        <div
                            className={`absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-gray-900 ${getStatusColor(friend.status)}`}
                        />
                    </div>
                    <div className="flex-1">
                        <h3 className="font-semibold text-gray-100">{friend.name}</h3>
                        <p className="text-sm text-gray-400">Level {friend.level}</p>
                        <div className="flex items-center gap-2 text-sm text-gray-400">
                            {friend.status === "active" ? (
                                <Gamepad2 className="h-3 w-3" />
                            ) : friend.status === "online" ? (
                                <Users className="h-3 w-3" />
                            ) : (
                                <Clock className="h-3 w-3" />
                            )}
                            <span>{getStatusText(friend)}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}