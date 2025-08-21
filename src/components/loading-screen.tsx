interface LoadingScreenProps {
    isVisible: boolean
    title?: string
    message?: string
}

export default function LoadingScreen({ 
    isVisible, 
    title = "Welcome Back!", 
    message = "Loading the library..." 
}: LoadingScreenProps) {
    return (
        <div
            className={`fixed inset-0 z-50 bg-gradient-to-br from-black via-gray-900 to-black transition-all duration-800 ${
                isVisible ? "opacity-100 scale-100" : "opacity-0 scale-110 pointer-events-none"
            }`}
        >
            <div className="flex items-center justify-center min-h-screen">
                <div className="text-center">
                    <div className="w-16 h-16 border-4 border-blue-500/30 border-t-blue-500 rounded-full animate-spin mx-auto mb-4"></div>
                    <h2 className="text-2xl font-bold text-gray-100 mb-2">{title}</h2>
                    <p className="text-gray-400">{message}</p>
                </div>
            </div>
        </div>
    )
}