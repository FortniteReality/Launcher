"use client"

import { useState } from "react"
import { Button } from "../../components/ui/button"
import {
  HardDrive,
  Folder,
} from "lucide-react"

function SettingsPage() {
  const [selectedSection, setSelectedSection] = useState("profile")

  const settingSections = [
    { id: "storage", label: "Storage", icon: HardDrive },
  ]

  const renderStorageSettings = () => (
    <div className="space-y-6">
      <div className="backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-6">
        <h3 className="text-lg font-semibold text-gray-100 mb-4">Storage Locations</h3>
        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 rounded-lg bg-black/20">
            <div className="flex items-center gap-3">
              <HardDrive className="h-5 w-5 text-gray-400" />
              <div>
                <p className="text-gray-100 font-medium">Primary Drive (C:)</p>
                <p className="text-sm text-gray-400">C:\Games</p>
              </div>
            </div>
            <div className="text-right">
              <p className="text-gray-100">2.1 TB free</p>
              <p className="text-sm text-gray-400">of 4.0 TB</p>
            </div>
          </div>
          <div className="flex items-center justify-between p-4 rounded-lg bg-black/20">
            <div className="flex items-center gap-3">
              <HardDrive className="h-5 w-5 text-gray-400" />
              <div>
                <p className="text-gray-100 font-medium">Secondary Drive (D:)</p>
                <p className="text-sm text-gray-400">D:\Games</p>
              </div>
            </div>
            <div className="text-right">
              <p className="text-gray-100">890 GB free</p>
              <p className="text-sm text-gray-400">of 2.0 TB</p>
            </div>
          </div>
        </div>
        <Button className="mt-4 bg-blue-600 hover:bg-blue-700">
          <Folder className="h-4 w-4 mr-2" />
          Add Storage Location
        </Button>
      </div>
    </div>
  )

  const renderContent = () => {
    switch (selectedSection) {
      case "storage":
        return renderStorageSettings()
      default:
        return (
          <div className="backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-6">
            <h3 className="text-lg font-semibold text-gray-100 mb-4">
              {settingSections.find((s) => s.id === selectedSection)?.label} Settings
            </h3>
            <p className="text-gray-400">Settings for this section are coming soon.</p>
          </div>
        )
    }
  }

  return (
    <div className="flex h-full">
      {/* Settings Sidebar */}
      <div className="w-64 backdrop-blur-xl bg-black/20 border-r border-gray-800/30 p-4 flex-shrink-0">
        <h2 className="text-xl font-bold text-gray-100 mb-4">Settings</h2>
        <nav className="space-y-1">
          {settingSections.map((section) => (
            <Button
              key={section.id}
              variant="ghost"
              className={`w-full justify-start gap-3 text-gray-300 hover:text-gray-100 hover:bg-black/20 ${
                selectedSection === section.id ? "bg-black/40 text-gray-100" : ""
              }`}
              onClick={() => setSelectedSection(section.id)}
            >
              <section.icon className="h-4 w-4" />
              {section.label}
            </Button>
          ))}
        </nav>
      </div>

      {/* Settings Content */}
      <div className="flex-1 overflow-y-auto p-6">
        <div className="max-w-4xl mx-auto">
          <div className="mb-6">
            <h1 className="text-2xl font-bold text-gray-100 mb-2">
              {settingSections.find((s) => s.id === selectedSection)?.label}
            </h1>
            <p className="text-gray-400">Manage your {selectedSection} preferences and settings.</p>
          </div>
          {renderContent()}
          <div className="flex justify-end gap-4 mt-8">
            <Button variant="outline" className="border-gray-700 text-gray-300 hover:bg-black/20 bg-transparent">
              Cancel
            </Button>
            <Button className="bg-blue-600 hover:bg-blue-700">Save Changes</Button>
          </div>
        </div>
      </div>
    </div>
  )
}

export default SettingsPage;