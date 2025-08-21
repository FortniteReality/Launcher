import { Button } from "./ui/button"
import { Avatar, AvatarFallback, AvatarImage } from "./ui/avatar"
import { Friend } from "../types/friend"

interface FriendRequestCardProps {
    request: Friend
    onAccept: (id: string) => Promise<void>
    onDecline: (id: string) => Promise<void>
}

export default function FriendRequestCard({ request, onAccept, onDecline }: FriendRequestCardProps) {
    return (
        <div className="backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-4 hover:bg-black/40 transition-all duration-300">
            <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                    <Avatar className="h-12 w-12">
                        <AvatarImage src={request.avatar || "/placeholder.svg"} />
                        <AvatarFallback className="bg-gray-800 text-gray-300">
                            {request.name
                                .split(" ")
                                .map((n) => n[0])
                                .join("")}
                        </AvatarFallback>
                    </Avatar>
                    <div>
                        <h3 className="font-semibold text-gray-100">{request.name}</h3>
                        <p className="text-sm text-gray-400">Level {request.level}</p>
                    </div>
                </div>
                <div className="flex items-center gap-2">
                    <Button 
                        size="sm" 
                        className="bg-green-600 hover:bg-green-700"
                        onClick={() => onAccept(request.id)}
                    >
                        Accept
                    </Button>
                    <Button 
                        size="sm" 
                        variant="outline" 
                        className="border-gray-700 text-gray-300 hover:bg-black/20 bg-transparent"
                        onClick={() => onDecline(request.id)}
                    >
                        Decline
                    </Button>
                </div>
            </div>
        </div>
    )
}