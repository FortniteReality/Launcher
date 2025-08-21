"use client"

import { useState } from "react"
import { Button } from "../../components/ui/button"
import { Input } from "../../components/ui/input"
import { Badge } from "../../components/ui/badge"
import { Search, Filter, Star, ShoppingCart, Heart, TrendingUp, Grid3X3, List } from "lucide-react"

interface StoreGame {
  id: string
  title: string
  image: string
  price: number
  originalPrice?: number
  genre: string
  rating: number
  reviews: number
  releaseDate: string
  developer: string
  tags: string[]
  featured: boolean
  onSale: boolean
  discount?: number
}

const storeGames: StoreGame[] = [
  {
    id: "1",
    title: "Starfield",
    image: "/placeholder.svg?height=300&width=400",
    price: 59.99,
    genre: "RPG",
    rating: 4.1,
    reviews: 12543,
    releaseDate: "Sep 6, 2023",
    developer: "Bethesda Game Studios",
    tags: ["Space", "Exploration", "RPG"],
    featured: true,
    onSale: false,
  },
  {
    id: "2",
    title: "Baldur's Gate 3",
    image: "/placeholder.svg?height=300&width=400",
    price: 39.99,
    originalPrice: 59.99,
    genre: "RPG",
    rating: 4.9,
    reviews: 89234,
    releaseDate: "Aug 3, 2023",
    developer: "Larian Studios",
    tags: ["RPG", "Turn-Based", "Fantasy"],
    featured: true,
    onSale: true,
    discount: 33,
  },
  {
    id: "3",
    title: "Spider-Man Remastered",
    image: "/placeholder.svg?height=300&width=400",
    price: 29.99,
    originalPrice: 49.99,
    genre: "Action",
    rating: 4.7,
    reviews: 45678,
    releaseDate: "Aug 12, 2022",
    developer: "Insomniac Games",
    tags: ["Action", "Adventure", "Superhero"],
    featured: false,
    onSale: true,
    discount: 40,
  },
  {
    id: "4",
    title: "Hogwarts Legacy",
    image: "/placeholder.svg?height=300&width=400",
    price: 49.99,
    genre: "RPG",
    rating: 4.4,
    reviews: 67890,
    releaseDate: "Feb 10, 2023",
    developer: "Avalanche Software",
    tags: ["RPG", "Magic", "Open World"],
    featured: false,
    onSale: false,
  },
]

function StorePage() {
  const [searchQuery, setSearchQuery] = useState("")
  const [viewMode, setViewMode] = useState<"grid" | "list">("grid")

  const filteredGames = storeGames.filter(
    (game) =>
      game.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      game.genre.toLowerCase().includes(searchQuery.toLowerCase()) ||
      game.tags.some((tag) => tag.toLowerCase().includes(searchQuery.toLowerCase())),
  )

  const featuredGames = storeGames.filter((game) => game.featured)

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="backdrop-blur-xl bg-black/20 border-b border-gray-800/30 p-4 flex-shrink-0">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4 flex-1">
            <div className="relative flex-1 max-w-md">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
              <Input
                placeholder="Search store..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-10 bg-black/20 border-gray-700 text-gray-100 placeholder:text-gray-400 focus:bg-black/30"
              />
            </div>
            <Button variant="ghost" size="icon" className="text-gray-400 hover:text-gray-100 hover:bg-black/20">
              <Filter className="h-4 w-4" />
            </Button>
          </div>

          <div className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="icon"
              className={`text-gray-400 hover:text-gray-100 hover:bg-black/20 ${viewMode === "grid" ? "bg-black/40 text-gray-100" : ""}`}
              onClick={() => setViewMode("grid")}
            >
              <Grid3X3 className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className={`text-gray-400 hover:text-gray-100 hover:bg-black/20 ${viewMode === "list" ? "bg-black/40 text-gray-100" : ""}`}
              onClick={() => setViewMode("list")}
            >
              <List className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-6">
        <div className="max-w-7xl mx-auto space-y-8">
          {/* Featured Section */}
          <div>
            <h2 className="text-2xl font-bold text-gray-100 mb-4 flex items-center gap-2">
              <TrendingUp className="h-6 w-6" />
              Featured Games
            </h2>
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              {featuredGames.map((game) => (
                <div
                  key={game.id}
                  className="backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 overflow-hidden hover:bg-black/40 transition-all duration-300"
                >
                  <div className="flex">
                    <img src={game.image || "/placeholder.svg"} alt={game.title} className="w-48 h-32 object-cover" />
                    <div className="flex-1 p-4">
                      <div className="flex items-start justify-between mb-2">
                        <div>
                          <h3 className="font-semibold text-gray-100 mb-1">{game.title}</h3>
                          <p className="text-sm text-gray-400">{game.developer}</p>
                        </div>
                        {game.onSale && (
                          <Badge className="bg-red-500/20 text-red-300 border-red-500/30">-{game.discount}%</Badge>
                        )}
                      </div>
                      <div className="flex items-center gap-2 mb-3">
                        <div className="flex items-center gap-1">
                          <Star className="h-3 w-3 fill-yellow-400 text-yellow-400" />
                          <span className="text-sm text-gray-300">{game.rating}</span>
                        </div>
                        <span className="text-gray-500">•</span>
                        <span className="text-sm text-gray-400">{game.reviews.toLocaleString()} reviews</span>
                      </div>
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          {game.originalPrice && (
                            <span className="text-sm text-gray-500 line-through">${game.originalPrice}</span>
                          )}
                          <span className="text-lg font-bold text-gray-100">${game.price}</span>
                        </div>
                        <Button size="sm" className="bg-blue-600 hover:bg-blue-700">
                          <ShoppingCart className="h-4 w-4 mr-1" />
                          Buy
                        </Button>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* All Games */}
          <div>
            <h2 className="text-2xl font-bold text-gray-100 mb-4">All Games</h2>
            {viewMode === "grid" ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                {filteredGames.map((game) => (
                  <div
                    key={game.id}
                    className="backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 overflow-hidden hover:bg-black/40 transition-all duration-300 hover:scale-105"
                  >
                    <div className="relative">
                      <img
                        src={game.image || "/placeholder.svg"}
                        alt={game.title}
                        className="w-full h-48 object-cover"
                      />
                      {game.onSale && (
                        <div className="absolute top-2 right-2">
                          <Badge className="bg-red-500/20 text-red-300 border-red-500/30">-{game.discount}%</Badge>
                        </div>
                      )}
                      <Button
                        size="icon"
                        variant="ghost"
                        className="absolute top-2 left-2 h-8 w-8 bg-black/50 hover:bg-black/70 text-gray-300"
                      >
                        <Heart className="h-4 w-4" />
                      </Button>
                    </div>
                    <div className="p-4">
                      <h3 className="font-semibold text-gray-100 mb-1 truncate">{game.title}</h3>
                      <p className="text-sm text-gray-400 mb-2">{game.developer}</p>
                      <div className="flex items-center gap-2 mb-3">
                        <div className="flex items-center gap-1">
                          <Star className="h-3 w-3 fill-yellow-400 text-yellow-400" />
                          <span className="text-sm text-gray-300">{game.rating}</span>
                        </div>
                        <Badge variant="outline" className="border-gray-700 text-gray-400 text-xs">
                          {game.genre}
                        </Badge>
                      </div>
                      <div className="flex items-center justify-between">
                        <div>
                          {game.originalPrice && (
                            <span className="text-sm text-gray-500 line-through">${game.originalPrice}</span>
                          )}
                          <span className="text-lg font-bold text-gray-100 ml-1">${game.price}</span>
                        </div>
                        <Button size="sm" className="bg-blue-600 hover:bg-blue-700">
                          Buy
                        </Button>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="space-y-3">
                {filteredGames.map((game) => (
                  <div
                    key={game.id}
                    className="backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-4 hover:bg-black/40 transition-all duration-300"
                  >
                    <div className="flex items-center gap-4">
                      <img
                        src={game.image || "/placeholder.svg"}
                        alt={game.title}
                        className="w-20 h-16 object-cover rounded-lg"
                      />
                      <div className="flex-1 min-w-0">
                        <h3 className="font-semibold text-gray-100 truncate">{game.title}</h3>
                        <p className="text-sm text-gray-400">{game.developer}</p>
                        <div className="flex items-center gap-4 text-sm text-gray-400 mt-1">
                          <div className="flex items-center gap-1">
                            <Star className="h-3 w-3 fill-yellow-400 text-yellow-400" />
                            <span>{game.rating}</span>
                          </div>
                          <span>{game.genre}</span>
                          <span>{game.releaseDate}</span>
                        </div>
                      </div>
                      <div className="flex items-center gap-4">
                        <div className="text-right">
                          {game.originalPrice && (
                            <p className="text-sm text-gray-500 line-through">${game.originalPrice}</p>
                          )}
                          <p className="text-lg font-bold text-gray-100">${game.price}</p>
                        </div>
                        <Button className="bg-blue-600 hover:bg-blue-700">
                          <ShoppingCart className="h-4 w-4 mr-2" />
                          Buy
                        </Button>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}

export default StorePage;