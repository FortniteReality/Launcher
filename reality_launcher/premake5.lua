include "./vendor/premake/premake_customization/solution_items.lua"

workspace "Reality"
	architecture "x64"
	startproject "RealityLauncher"
	
	configurations
	{
		"Debug",
		"Release"
	}

group "Launcher"
	include "RealityLauncher"
group ""