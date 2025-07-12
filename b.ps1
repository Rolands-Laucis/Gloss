# optimized build command for Tauri
npm run tauri build -- --no-bundle

# Only move the files if the build didn't error
if ($?) {
    Write-Output "deleting unneeded build folders..."
    # Delete specific folders if they exist
    # $foldersToDelete = @("build", "bundle", "examples", "incremental", "nsis", "wix", "deps")
    $foldersToDelete = @("build","bundle", "examples", "incremental", "nsis", "wix")
    foreach ($folder in $foldersToDelete) {
        $path = "src-tauri\target\release\$folder"
        if (Test-Path $path) {
            Remove-Item $path -Recurse -Force
        }
    }

    Write-Output "moving release folder..."
    Copy-Item src-tauri\target\release -Destination F:\Gloss -Recurse -Force

    $foldersToDelete = @("build","deps")
    foreach ($folder in $foldersToDelete) {
        $path = "F:\Gloss\release\$folder"
        if (Test-Path $path) {
            Remove-Item $path -Recurse -Force
        }
    }
}