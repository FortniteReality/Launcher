"use client"

import { useState, useEffect } from "react"
import { Button } from "../../components/ui/button"
import { Input } from "../../components/ui/input"
import { Badge } from "../../components/ui/badge"
import { Search, RefreshCw } from "lucide-react"
import { Friend } from "../../types/friend"
import { friendsApi } from "../../services/friends-api"
import { filterFriends } from "../../utils/friend-utils"
import FriendCard from "../../components/friend-card"
import FriendRequestCard from "../../components/friend-request-card"

function FriendsPage() {
    // UI state
    const [searchQuery, setSearchQuery] = useState("")
    const [selectedTab, setSelectedTab] = useState("friends")
    const [isLoading, setIsLoading] = useState(true)
    const [isRefreshing, setIsRefreshing] = useState(false)

    // Friends data
    const [friends, setFriends] = useState<Friend[]>([])
    const [friendRequests, setFriendRequests] = useState<Friend[]>([])

    // Computed values
    const filteredFriends = filterFriends(friends, searchQuery)

    // Load initial data
    useEffect(() => {
        const timer = setTimeout(() => {
            loadInitialData()
        }, 100)

        return () => clearTimeout(timer)
    }, [])

    // Update filtered friends when search query changes
    useEffect(() => {
        // This effect ensures the filtered friends update when searchQuery changes
        // The filteredFriends computed value handles this automatically
    }, [searchQuery, friends])

    const loadInitialData = async () => {
        setIsLoading(true)
        try {
            const [friendsData, requestsData] = await Promise.all([
                friendsApi.fetchFriendsList(),
                friendsApi.fetchFriendRequests()
            ])
            
            setFriends(friendsData)
            setFriendRequests(requestsData)
        } catch (error) {
            console.error("Error loading initial data:", error)
        } finally {
            setIsLoading(false)
        }
    }

    const refreshFriendRequests = async () => {
        setIsRefreshing(true)
        try {
            const requestsData = await friendsApi.fetchFriendRequests()
            setFriendRequests(requestsData)
        } catch (error) {
            console.error("Error refreshing friend requests:", error)
        } finally {
            setIsRefreshing(false)
        }
    }

    const handleAcceptFriendRequest = async (friendAccountId: string) => {
        try {
            const newFriend = await friendsApi.acceptFriendRequest(friendAccountId)
            
            // Add to friends list
            setFriends(prevFriends => [...prevFriends, newFriend])
            
            // Remove from friend requests
            setFriendRequests(prevRequests => 
                prevRequests.filter(request => request.id !== friendAccountId)
            )
        } catch (error) {
            console.error("Error accepting friend request:", error)
        }
    }

    const handleDeclineFriendRequest = async (friendAccountId: string) => {
        try {
            await friendsApi.declineFriendRequest(friendAccountId)
            
            // Remove from friend requests
            setFriendRequests(prevRequests => 
                prevRequests.filter(request => request.id !== friendAccountId)
            )
        } catch (error) {
            console.error("Error declining friend request:", error)
        }
    }

    if (isLoading) {
        return (
            <div className="flex items-center justify-center h-full">
                <div className="text-gray-400">Loading friends...</div>
            </div>
        )
    }

    return (
        <div className="flex flex-col h-full">
            <div className="backdrop-blur-xl bg-black/20 border-b border-gray-800/30 p-4 flex-shrink-0">
                <div className="flex items-center justify-between">
                    <div className="flex items-center gap-4">
                        <h1 className="text-2xl font-bold text-gray-100">Friends</h1>
                        <Badge variant="outline" className="border-gray-700 text-gray-300">
                            {filteredFriends.length} friends
                        </Badge>
                    </div>
                </div>

                <div className="flex items-center gap-4 mt-4">
                    {[
                        { id: "friends", label: "All Friends", count: filteredFriends.length },
                        { id: "online", label: "Online", count: friends.filter((f) => f.status !== "offline").length },
                        { id: "requests", label: "Requests", count: friendRequests.length },
                    ].map((tab) => (
                        <Button
                            key={tab.id}
                            variant="ghost"
                            className={`text-gray-300 hover:text-gray-100 hover:bg-black/20 ${
                                selectedTab === tab.id ? "bg-black/40 text-gray-100" : ""
                            }`}
                            onClick={() => setSelectedTab(tab.id)}
                        >
                            {tab.label} ({tab.count})
                        </Button>
                    ))}
                </div>
            </div>

            <div className="flex-1 overflow-y-auto p-6">
                <div className="max-w-4xl mx-auto">
                    {selectedTab !== "requests" && (
                        <div className="relative mb-6">
                            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
                            <Input
                                placeholder="Search friends..."
                                value={searchQuery}
                                onChange={(e) => setSearchQuery(e.target.value)}
                                className="pl-10 bg-black/20 border-gray-700 text-gray-100 placeholder:text-gray-400 focus:bg-black/30"
                            />
                        </div>
                    )}

                    {selectedTab === "requests" && (
                        <div className="space-y-4">
                            <div className="flex items-center justify-between mb-4">
                                <h2 className="text-xl font-semibold text-gray-100">Friend Requests</h2>
                                <Button
                                    variant="outline"
                                    size="sm"
                                    onClick={refreshFriendRequests}
                                    disabled={isRefreshing}
                                    className="border-gray-700 text-gray-300 hover:bg-black/20 bg-transparent"
                                >
                                    <RefreshCw className={`h-4 w-4 mr-2 ${isRefreshing ? 'animate-spin' : ''}`} />
                                    Refresh
                                </Button>
                            </div>
                            
                            {friendRequests.length === 0 ? (
                                <div className="text-center py-8 text-gray-400">
                                    No friend requests
                                </div>
                            ) : (
                                friendRequests.map((request) => (
                                    <FriendRequestCard
                                        key={request.id}
                                        request={request}
                                        onAccept={handleAcceptFriendRequest}
                                        onDecline={handleDeclineFriendRequest}
                                    />
                                ))
                            )}
                        </div>
                    )}

                    {(selectedTab === "friends" || selectedTab === "online") && (
                        <div className="space-y-4">
                            {filteredFriends
                                .filter((friend) => selectedTab === "friends" || friend.status !== "offline")
                                .length === 0 ? (
                                <div className="text-center py-8 text-gray-400">
                                    {selectedTab === "online" ? "No friends online" : "No friends found"}
                                </div>
                            ) : (
                                filteredFriends
                                    .filter((friend) => selectedTab === "friends" || friend.status !== "offline")
                                    .map((friend) => (
                                        <FriendCard key={friend.id} friend={friend} />
                                    ))
                            )}
                        </div>
                    )}
                </div>
            </div>
        </div>
    )
}

export default FriendsPage