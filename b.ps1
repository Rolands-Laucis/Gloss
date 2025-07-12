# Corrected build command for Tauri
npm run tauri build -- --no-bundle

# Only move the files if the build didn't error
if ($?) {
    Write-Output "deleting unneeded build folders..."
    # Delete specific folders if they exist
    # $foldersToDelete = @("build", "bundle", "examples", "incremental", "nsis", "wix", "deps")
    $foldersToDelete = @("bundle", "examples", "incremental", "nsis", "wix", "deps")
    foreach ($folder in $foldersToDelete) {
        $path = "src-tauri\target\release\$folder"
        if (Test-Path $path) {
            Remove-Item $path -Recurse -Force
        }
    }

    Write-Output "moving release folder..."
    Copy-Item src-tauri\target\release -Destination F:\Gloss -Recurse -Force
    
    if (Test-Path "F:\Gloss\release\build") {
        Remove-Item "F:\Gloss\release\build" -Recurse -Force
    }
}