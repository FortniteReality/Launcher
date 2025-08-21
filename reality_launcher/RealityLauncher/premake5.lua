project "RealityLauncher"
	kind "WindowedApp"
	language "C#"
	dotnetframework "net8.0"
	targetdir ("bin/%{cfg.buildcfg}")
	objdir ("bin-int/%{cfg.buildcfg}")
	icon "Reality-Logo.ico"

	files 
	{
		"**.cs",
		"**.ico"
	}

	links
	{
		"System",
		"System.Core",
		"System.Xml",
		"System.Data",
	}

    filter "configurations:Debug"
        symbols "On"

    filter "configurations:Release"
        optimize "On"