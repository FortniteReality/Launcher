import { useState, useEffect } from 'react'
import SignInPage from '../launcher/pages/sign-in'
import Launcher from '../launcher/game-launcher'
function App() {
  const [currentView, setCurrentView] = useState<'signin' | 'launcher'>('signin')
  useEffect(() => {
    // Check if we should show launcher on app start
    try {
      const showLauncher = sessionStorage.getItem('showLauncher')
      if (showLauncher === 'true') {
        setCurrentView('launcher')
        sessionStorage.removeItem('showLauncher') // Clean up
      }
    } catch (error) {
      // sessionStorage might not be available
    }
    // Listen for navigation events
    const handleNavigateToLauncher = () => {
      setCurrentView('launcher')
    }
    window.addEventListener('navigate-to-launcher', handleNavigateToLauncher)
    return () => {
      window.removeEventListener('navigate-to-launcher', handleNavigateToLauncher)
    }
  }, [])
  if (currentView === 'launcher') {
    return <Launcher />
  }
  return <SignInPage />
}
export default App